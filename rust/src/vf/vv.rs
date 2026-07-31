
use crate::support::primitives::{Pos};

// 要素(`Pos`)は所有物なしのプリミティブなので Vec 化に伴う所有権の問題は無い。
// vtableの生存スロット(`.init`/`.push`/`.shrink_to_fit`/`.dispose`)は呼び出し側
// (`table/fvar.rs`)で直接 `Vec` のメソッドに置き換えた（詳細は `vf/vq.rs`）。
pub type VV = Vec<Pos>;
