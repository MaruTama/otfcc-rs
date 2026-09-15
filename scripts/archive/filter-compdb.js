// Reduce a full ninja compilation database to the single release-x64 C config.
// c2rust transpiles every entry it is given; without this it would translate
// the same sources four times (debug/release x64/x86).
//
// Usage: node filter-compdb.js <in.json> <out.json>
var fs = require("fs");
var input = process.argv[2];
var output = process.argv[3];

var all = JSON.parse(fs.readFileSync(input, "utf8"));
var seen = new Set();
var out = [];
for (var i = 0; i < all.length; i++) {
	var e = all[i];
	if (!/obj\/x64\/release\//.test(e.command)) continue;
	if (!e.file.endsWith(".c")) continue;
	if (seen.has(e.file)) continue;
	seen.add(e.file);
	out.push(e);
}
fs.writeFileSync(output, JSON.stringify(out, null, 2));
process.stderr.write("release-x64 C translation units: " + out.length + "\n");
