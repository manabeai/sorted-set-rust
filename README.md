# sorted-set (Rust)

[tatyam-prime/SortedSet](https://github.com/tatyam-prime/SortedSet) の Rust 移植です。
平衡木ではなく、配列をバケットに分割して管理します。依存クレートはありません。

- `SortedSet<T>`: 重複を除く順序付き集合
- `SortedMultiset<T>`: 重複を許す順序付き多重集合
- `BucketList<T>`: 挿入順を保持するバケット方式のリスト

## コピペで使う

必要な型のファイル全体を、そのまま提出コードのトップレベルへ貼り付けてください。
標準ライブラリだけで動作し、`lib.rs` や他の実装ファイルは不要です。
複数の型を使う場合も、それぞれの内容を同じコードに貼り付けられます。

| 型 | 単独で使えるファイル |
| --- | --- |
| `SortedSet` | [src/sorted_set.rs](src/sorted_set.rs) |
| `SortedMultiset` | [src/sorted_multiset.rs](src/sorted_multiset.rs) |
| `BucketList` | [src/bucket_list.rs](src/bucket_list.rs) |

```rust
// src/sorted_set.rs の内容をここに貼り付ける
fn main() {
    let mut s = SortedSet::new();
    s.add(3);
    s.add(1);
    assert_eq!(s[-1], 3);
}
```

公開関数には `///` 形式のドキュメントコメントを付けています。

## クレートとして導入

Rust stable / Edition 2021 を使用します。crates.io には未公開です。

```toml
[dependencies]
sorted-set = { git = "https://github.com/manabeai/sorted-set-rust" }
```

ローカルで使う場合:

```toml
[dependencies]
sorted-set = { path = "../sorted-set-rust" }
```

```rust
use sorted_set::{SortedSet, SortedMultiset};

let mut s: SortedSet<_> = [3, 1, 3, 2].into_iter().collect();
assert!(s.add(4));
assert_eq!(s.lt(&3), Some(&2));
assert_eq!(s.index(&3), 2);
assert_eq!(s[-1], 4);
assert_eq!(s.pop(-1), Some(4));

let mut m: SortedMultiset<_> = [2, 2, 1].into_iter().collect();
assert!(m.discard(&2)); // 1 個だけ削除
assert_eq!(m.count(&2), 1);
```

## ドキュメント・検証

- [API リファレンス](docs/API.md): 全操作、計算量、Python 版との対応
- [実行可能な使用例](examples/basic.rs)
- `cargo doc --no-deps`: HTML ドキュメントを `target/doc/sorted_set/index.html` に生成

```sh
cargo test
python3 tests/standalone.py
cargo clippy --all-targets -- -D warnings
cargo run --example basic
cargo doc --no-deps
```

テストはソート済み配列とのランダム比較、重複、負の添字、バケット分割・削除、空集合への再挿入を含みます。GitHub Actions でも検証します。

## 出典とライセンス

移植元: [tatyam-prime/SortedSet](https://github.com/tatyam-prime/SortedSet/tree/9a205c686c225c2b083c4852516da34a6ca1f4c2)
（コミット `9a205c686c225c2b083c4852516da34a6ca1f4c2`）。
元実装の `BUCKET_RATIO = 16`、`SPLIT_RATIO = 24` と分割方式を使用しています。
本実装も [Unlicense](LICENSE) です。
