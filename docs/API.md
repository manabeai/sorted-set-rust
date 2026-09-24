# Rust API

```rust
use sorted_set::{SortedSet, SortedMultiset, BucketList};
```

## 共通規則

`SortedSet<T>` / `SortedMultiset<T>` は `T: Ord` が必要です。
`BucketList<T>` の基本操作には比較可能性は不要で、検索・個数取得・値による削除のみ `T: PartialEq` が必要です。
要素の `Clone` は通常の操作に不要です。コンテナを `clone()` する場合は `T: Clone` が必要です。

添字は `isize` です。`0` は先頭、`-1` は末尾、`-len` は先頭です。
`get` / `get_mut` / `pop` は範囲外で `None` を返し、変更しません。
`s[i]` は範囲外で panic します。スライス指定には対応しません。

バケットは `buckets()` から読み取り専用で確認できます。集合内の値を変更すると順序が壊れるため、集合の可変参照は公開しません。
`Cell` などを使って格納中の要素の比較結果を変えないでください。
走査中の変更は Rust の借用規則で禁止されます。

## SortedSet / SortedMultiset

各型は別々の自己完結した実装です。`src/sorted_set.rs` と `src/sorted_multiset.rs` は、それぞれ単独でコピペして利用できます。

| 操作 | 内容 |
| --- | --- |
| `SortedSet::new()` / `default()` | 空の集合 |
| `iter.collect::<SortedSet<_>>()` | iterable から構築。必要な場合のみソートし、集合では重複除去 |
| `len() -> usize` / `is_empty() -> bool` | 要素数 / 空判定 |
| `contains(&x) -> bool` | 存在判定 |
| `add(x) -> bool` | 集合は新規追加なら `true`、既存なら `false`。多重集合は必ず追加し `true` |
| `discard(&x) -> bool` | 1 個だけ削除。存在しなければ `false` |
| `lt(&x) -> Option<&T>` | `x` 未満の最大要素 |
| `le(&x) -> Option<&T>` | `x` 以下の最大要素 |
| `gt(&x) -> Option<&T>` | `x` より大きい最小要素 |
| `ge(&x) -> Option<&T>` | `x` 以上の最小要素 |
| `index(&x) -> usize` | `x` 未満の要素数。`x` が存在しなくても使える |
| `index_right(&x) -> usize` | `x` 以下の要素数 |
| `count(&x) -> usize` | `x` の個数（集合にも提供） |
| `get(i) -> Option<&T>` / `s[i]` | 昇順で `i` 番目の値。負の添字に対応 |
| `pop(i) -> Option<T>` | `i` 番目を削除して所有権を返す |
| `pop_last() -> Option<T>` | 末尾を削除して返す。Python の引数なし `pop()` に相当 |
| `iter()` / `for x in &s` | 昇順の `&T` を返す |
| `iter().rev()` | 降順の `&T` を返す |
| `extend(iter)` | 各値を `add` する |
| `clear()` | 空にする |
| `clone()` | 独立したコピー（`T: Clone`） |
| `==` / `!=` | 値の並びで比較。バケットの分け方には依存しない |
| `buckets() -> &[Vec<T>]` | 内部バケットの読み取り専用ビュー |

比較は `Ord` に従います。降順にしたい場合は `std::cmp::Reverse<T>` を使用できます。
浮動小数点数はそのままでは `Ord` を実装しないため、全順序を定義した型が必要です。

## BucketList

構築時にソートや重複除去を行いません。

| 操作 | 内容 |
| --- | --- |
| `new()` / `default()` / `iter.collect::<BucketList<_>>()` | 構築 |
| `len()` / `is_empty()` / `buckets()` | 要素数、空判定、バケット参照 |
| `get(i) -> Option<&T>` | 添字参照 |
| `get_mut(i) -> Option<&mut T>` / `s[i] = x` | 要素の変更 |
| `insert(i, x) -> Result<(), T>` | `i` 番目の直前に挿入。範囲外では `Err(x)` で値を返す |
| `append(x)` / `extend(iter)` | 末尾に追加 |
| `pop(i)` / `pop_last()` | 添字 / 末尾で削除。`Option<T>` |
| `contains(&x) -> bool` / `count(&x) -> usize` | 存在判定 / 個数 |
| `index(&x) -> Option<usize>` | 最初に一致する位置。集合の `index` と意味が異なる |
| `remove(&x) -> Option<T>` | 最初の一致を削除。なければ `None` |
| `reverse()` | その場で反転 |
| `clear()` / `clone()` | 空にする / コピー |
| `iter()` / `iter().rev()` / `for x in &s` | 順方向 / 逆方向の走査 |
| `==` / `!=` | 要素の並びで比較 |

`insert` の有効範囲は、空でない場合 `-len..=len` です。
`insert(-1, x)` は末尾要素の直前、`insert(len, x)` は末尾に挿入します。
空の場合は移植元に合わせて `0` と `-1` のみ受け付けます。
Python 標準 `list.insert` と異なり、範囲外を端に丸めません。

## 計算量と方式

`N` を要素数、`B` をバケット数、`K` を最大バケット長とします。

| 操作 | 計算量 |
| --- | --- |
| 集合の構築 | ソート済みなら O(N)、それ以外は O(N log N) |
| リストの構築 | O(N) |
| 要素数・空判定 | O(1) |
| 集合の検索・近傍検索・順位・個数 | O(B + log K) |
| 添字参照 | O(B)。負の添字では末尾から探す |
| 挿入・添字削除 | O(B + K)。配列の再確保コストは償却 |
| 集合の値による削除 | O(B + K) |
| リストの値による検索・削除・個数 | O(N) |
| 全走査・等値比較・コピー・反転 | O(N) |
| メモリ | O(N) |

構築時は約 `ceil(sqrt(N / 16))` 個のバケットを作り、バケット長が `24 * B` を超えると二分割します。
空バケットは削除します。通常は平方根分割として O(√N) 程度の操作になりますが、削除後の全体再構築は移植元と同様に行いません。
過去の大量追加・削除によって分布が偏ることがあり、現在の `N` だけによる厳密な最悪 O(√N) を保証するものではありません。

## Python 版との違い

- `None` / `IndexError` / `ValueError` の代わりに、通常の失敗は `Option` / `Result` で表現します。添字演算子は panic します。
- Rust には省略可能引数がないので、末尾削除は `pop_last()` または `pop(-1)` です。
- `SortedMultiset.add` は Python の `None` ではなく `true` を返します。
- Python の `copy()` は `clone()`、`reversed(s)` は `s.iter().rev()` です。
- Python の文字列表示は再現せず、デバッグ表示は `Debug` を使用します。
- 内部バケットを直接変更する API、集合の可変添字、スライス操作は提供しません。
