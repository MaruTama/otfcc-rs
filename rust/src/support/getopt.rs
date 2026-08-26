//! A hand-rolled, GNU `getopt_long`-compatible argument parser.
//!
//! Replaces the FFI binding this file used to hold: `libc` declares
//! `struct option`/`getopt_long` for the BSDs, Apple, Solaris, Android and
//! Hurd, but **not** for `*-unknown-linux-gnu` -- so real `getopt_long` was
//! never portable to this crate's own CI target in the first place. This
//! reimplements the two behaviors that made it worth having (over a plain
//! positional scan): permuting options in front of positional arguments
//! regardless of where they appear in argv, and matching an unambiguous
//! prefix of a long option's name. Both are pinned by
//! `rust/tests/getopt.rs`, added before this file was rewritten specifically
//! so the behavior being replaced was captured first, not reconstructed from
//! memory of what `getopt_long` does.
//!
//! Deliberately narrower than the real thing: no `-W longopt` extension, no
//! `optstring` leading `+`/`-` mode switches, no `POSIXLY_CORRECT` handling
//! -- this crate's two binaries never used any of those, and glibc's own
//! `getopt_long` documentation calls them all rarely-used corners even in C
//! code.

/// One long option's spec. `short`, when set, is the character `optstring`
/// would have paired with it in the old `getopt_long` call -- both spellings
/// resolve to the same [`GetoptItem::Opt`] `val`, matching how the original
/// `longopts` arrays in `bin/otfccbuild.rs`/`bin/otfccdump.rs` reused a
/// short option's own char as that entry's `val` instead of `0`.
pub struct LongOpt {
    pub name: &'static str,
    pub has_arg: bool,
    /// Matches the real `getopt_long`'s `val`: this option's `has_arg`,
    /// stored again here as a plain identifier rather than as `flag`/`val`'s
    /// split roles, since neither binary ever used `flag`.
    pub val: i32,
}

/// One parsed step: an option (with the char/val that identifies which one,
/// and its argument if it takes one), a positional argument, or one of a
/// handful of error shapes -- each already carrying the exact message text
/// `bin/otfccbuild.rs`/`bin/otfccdump.rs` print for it, matching what real
/// `getopt_long` itself would have written to stderr for the same input.
pub enum GetoptItem {
    Opt { val: i32, arg: Option<String> },
    UnknownLong(String),
    UnknownShort(char),
    AmbiguousLong { given: String, matches: Vec<&'static str> },
    MissingArgument(String),
}

/// A single short option's own entry, split out of `optstring` the way the
/// original C code spelled it (`"vhqskiO:o:"`) -- `':'` immediately after a
/// char means that option takes a required argument, same convention.
fn parse_optstring(optstring: &str) -> Vec<(char, bool)> {
    let mut chars = optstring.chars().peekable();
    let mut out = Vec::new();
    while let Some(c) = chars.next() {
        let has_arg = chars.peek() == Some(&':');
        if has_arg {
            chars.next();
        }
        out.push((c, has_arg));
    }
    out
}

/// Finds `name` (or its unambiguous prefix) among `longopts`, GNU-style: an
/// exact match always wins outright (even if it's also a prefix of some
/// longer option name -- matching real `getopt_long`, `--time` is never
/// "ambiguous" even if some other option were named `--timezone`), otherwise
/// every option whose name starts with `name` is a candidate, and there
/// must be exactly one.
fn resolve_long<'a>(name: &str, longopts: &'a [LongOpt]) -> Result<&'a LongOpt, Vec<&'static str>> {
    if let Some(exact) = longopts.iter().find(|o| o.name == name) {
        return Ok(exact);
    }
    let matches: Vec<&LongOpt> = longopts.iter().filter(|o| o.name.starts_with(name)).collect();
    match matches.len() {
        1 => Ok(matches[0]),
        0 => Err(Vec::new()),
        _ => Err(matches.iter().map(|o| o.name).collect()),
    }
}

/// Parses `args` (typically `std::env::args().skip(1).collect()`) against
/// `optstring`/`longopts`, GNU `getopt_long`-style: returns every option in
/// the order encountered, *permuted* in front of every positional argument
/// regardless of where either appeared in `args` (GNU's signature departure
/// from POSIX `getopt`, which stops at the first non-option argument
/// instead) -- followed by the positional arguments themselves, in their
/// own relative order. A literal `--` argument ends option parsing; every
/// argument after it is positional even if it looks like an option.
///
/// Returns the parsed items and the leftover positional arguments together,
/// rather than as a stateful iterator plus an `optind`-style index the
/// caller reads afterward -- there is no C-side global state left to mirror
/// once this isn't crossing an FFI boundary.
pub fn getopt_long(
    args: &[String],
    optstring: &str,
    longopts: &[LongOpt],
) -> (Vec<GetoptItem>, Vec<String>) {
    let shortopts = parse_optstring(optstring);
    let mut items = Vec::new();
    let mut positionals = Vec::new();
    let mut i = 0;
    let mut options_ended = false;
    while i < args.len() {
        let arg = &args[i];
        if options_ended || arg == "-" || !arg.starts_with('-') {
            positionals.push(arg.clone());
            i += 1;
            continue;
        }
        if arg == "--" {
            options_ended = true;
            i += 1;
            continue;
        }
        if let Some(rest) = arg.strip_prefix("--") {
            let (name, inline_val) = match rest.split_once('=') {
                Some((n, v)) => (n, Some(v.to_string())),
                None => (rest, None),
            };
            match resolve_long(name, longopts) {
                Ok(opt) => {
                    if opt.has_arg {
                        if let Some(v) = inline_val {
                            items.push(GetoptItem::Opt { val: opt.val, arg: Some(v) });
                        } else if i + 1 < args.len() {
                            i += 1;
                            items.push(GetoptItem::Opt {
                                val: opt.val,
                                arg: Some(args[i].clone()),
                            });
                        } else {
                            items.push(GetoptItem::MissingArgument(format!("--{name}")));
                        }
                    } else {
                        items.push(GetoptItem::Opt { val: opt.val, arg: None });
                    }
                }
                Err(matches) if matches.is_empty() => {
                    items.push(GetoptItem::UnknownLong(format!("--{name}")));
                }
                Err(matches) => {
                    items.push(GetoptItem::AmbiguousLong {
                        given: format!("--{name}"),
                        matches,
                    });
                }
            }
            i += 1;
            continue;
        }
        // Short option(s): `-` followed by one or more chars, e.g. `-vh` (two
        // flags bundled) or `-O2`/`-O 2` (a value-taking option, inline or
        // as the next argument) -- once a value-taking short option is hit
        // mid-bundle, the rest of *this* token is its argument, matching
        // real `getopt_long` (so `-qO2` is `-q -O2`, not `-q -O -2`).
        let flags: Vec<char> = arg[1..].chars().collect();
        let mut j = 0;
        while j < flags.len() {
            let flag = flags[j];
            match shortopts.iter().find(|(c, _)| *c == flag) {
                Some((_, has_arg)) if *has_arg => {
                    let rest: String = flags[j + 1..].iter().collect();
                    if !rest.is_empty() {
                        items.push(GetoptItem::Opt {
                            val: flag as i32,
                            arg: Some(rest),
                        });
                    } else if i + 1 < args.len() {
                        i += 1;
                        items.push(GetoptItem::Opt {
                            val: flag as i32,
                            arg: Some(args[i].clone()),
                        });
                    } else {
                        items.push(GetoptItem::MissingArgument(format!("-{flag}")));
                    }
                    break;
                }
                Some(_) => {
                    items.push(GetoptItem::Opt { val: flag as i32, arg: None });
                    j += 1;
                }
                None => {
                    items.push(GetoptItem::UnknownShort(flag));
                    j += 1;
                }
            }
        }
        i += 1;
    }
    (items, positionals)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    const DEMO_LONGOPTS: &[LongOpt] = &[
        LongOpt { name: "verbose", has_arg: false, val: 0 },
        LongOpt { name: "version", has_arg: false, val: 'v' as i32 },
        LongOpt { name: "value", has_arg: true, val: 0 },
        LongOpt { name: "value-other", has_arg: true, val: 0 },
    ];
    const DEMO_OPTSTRING: &str = "vo:";

    #[test]
    fn permutes_positionals_before_and_after_options() {
        let (items, positionals) =
            getopt_long(&args(&["file.txt", "-v"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert_eq!(positionals, vec!["file.txt".to_string()]);
        assert!(matches!(items[0], GetoptItem::Opt { val, .. } if val == 'v' as i32));
    }

    #[test]
    fn unambiguous_long_prefix_matches() {
        let (items, _) = getopt_long(&args(&["--verb"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(items[0], GetoptItem::Opt { val: 0, arg: None }));
    }

    #[test]
    fn exact_long_match_wins_even_when_also_a_prefix_of_another() {
        // "value" is itself a full option name, and also a prefix of
        // "value-other" -- the exact match must win outright, not be
        // reported as ambiguous.
        let (items, _) = getopt_long(&args(&["--value=x"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(&items[0], GetoptItem::Opt { val: 0, arg: Some(v) } if v == "x"));
    }

    #[test]
    fn ambiguous_long_prefix_is_reported_not_guessed() {
        // "val" prefixes both "value" and "value-other" -- neither is an
        // exact match, so this must be reported ambiguous rather than
        // silently picking one.
        let (items, _) = getopt_long(&args(&["--val"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(&items[0], GetoptItem::AmbiguousLong { given, .. } if given == "--val"));
    }

    #[test]
    fn long_option_with_required_arg_takes_next_token() {
        let (items, positionals) =
            getopt_long(&args(&["--value", "x", "file"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(&items[0], GetoptItem::Opt { val: 0, arg: Some(v) } if v == "x"));
        assert_eq!(positionals, vec!["file".to_string()]);
    }

    #[test]
    fn short_option_with_attached_value() {
        let (items, _) = getopt_long(&args(&["-o2"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(&items[0], GetoptItem::Opt { val, arg: Some(v) } if *val == 'o' as i32 && v == "2"));
    }

    #[test]
    fn short_option_with_separate_value() {
        let (items, _) = getopt_long(&args(&["-o", "2"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(&items[0], GetoptItem::Opt { val, arg: Some(v) } if *val == 'o' as i32 && v == "2"));
    }

    #[test]
    fn bundled_short_flags_split_into_separate_opts() {
        let (items, _) = getopt_long(&args(&["-vv"]), "vv", &[]);
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|it| matches!(it, GetoptItem::Opt { val, arg: None } if *val == 'v' as i32)));
    }

    #[test]
    fn value_taking_short_mid_bundle_consumes_the_rest_of_the_token() {
        // "-qo2": -q is a flag, then -o takes "2" as its own value, not "2"
        // reparsed as more short flags.
        let (items, _) = getopt_long(&args(&["-qo2"]), "qo:", &[]);
        assert!(matches!(items[0], GetoptItem::Opt { val, arg: None } if val == 'q' as i32));
        assert!(matches!(&items[1], GetoptItem::Opt { val, arg: Some(v) } if *val == 'o' as i32 && v == "2"));
    }

    #[test]
    fn double_dash_ends_option_parsing() {
        let (items, positionals) =
            getopt_long(&args(&["-v", "--", "-v", "file"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert_eq!(items.len(), 1);
        assert_eq!(positionals, vec!["-v".to_string(), "file".to_string()]);
    }

    #[test]
    fn unknown_long_option_is_reported_and_parsing_continues() {
        let (items, positionals) =
            getopt_long(&args(&["--bogus", "file"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(&items[0], GetoptItem::UnknownLong(s) if s == "--bogus"));
        assert_eq!(positionals, vec!["file".to_string()]);
    }

    #[test]
    fn unknown_short_option_is_reported_and_parsing_continues() {
        let (items, positionals) =
            getopt_long(&args(&["-Z", "file"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(items[0], GetoptItem::UnknownShort('Z')));
        assert_eq!(positionals, vec!["file".to_string()]);
    }

    #[test]
    fn missing_required_argument_is_reported_not_a_panic() {
        let (items, _) = getopt_long(&args(&["-o"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(matches!(&items[0], GetoptItem::MissingArgument(s) if s == "-o"));
    }

    #[test]
    fn a_lone_dash_is_positional_not_an_option() {
        // Conventional stdin/stdout placeholder in many CLI tools' argv
        // grammars; real getopt_long treats it as a non-option argument.
        let (items, positionals) = getopt_long(&args(&["-"]), DEMO_OPTSTRING, DEMO_LONGOPTS);
        assert!(items.is_empty());
        assert_eq!(positionals, vec!["-".to_string()]);
    }
}
