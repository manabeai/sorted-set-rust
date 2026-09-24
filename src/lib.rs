//! Bucket-based containers ported from <https://github.com/tatyam-prime/SortedSet>.
//! Negative indices count from the end. Missing values and invalid indices return `None`.
//! Sorted containers deliberately expose only immutable element references.
//!
//! ```
//! use sorted_set::SortedSet;
//! let mut s: SortedSet<_> = [3, 1, 2, 1].into_iter().collect();
//! assert_eq!(s.index(&3), 2);
//! assert_eq!(s.lt(&3), Some(&2));
//! assert_eq!(s.pop_last(), Some(3));
//! ```
#![deny(missing_docs)]

/// 挿入順を保持するバケット方式のリスト。
#[derive(Clone, Debug)]
pub struct BucketList<T> {
    buckets: Vec<Vec<T>>,
    len: usize,
}

impl<T> Default for BucketList<T> {
    /// 空のコンテナを作ります。
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BucketList<T> {
    /// 空のコンテナを作ります。O(1)。
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
    }
    /// 格納している要素数を返します。重複も数えます。O(1)。
    pub fn len(&self) -> usize {
        self.len
    }
    /// 要素が存在しない場合に `true` を返します。O(1)。
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// 内部バケットの読み取り専用ビューを返します。空のバケットは含みません。O(1)。
    pub fn buckets(&self) -> &[Vec<T>] {
        &self.buckets
    }
    /// 要素への読み取り専用の双方向イテレータを返します。`rev()` で逆順に走査できます。
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.buckets.iter().flatten()
    }
    /// すべての要素を削除して空にします。
    pub fn clear(&mut self) {
        self.buckets.clear();
        self.len = 0;
    }

    fn locate(&self, mut index: isize) -> Option<(usize, usize)> {
        if index < 0 {
            for (b, bucket) in self.buckets.iter().enumerate().rev() {
                index += bucket.len() as isize;
                if index >= 0 {
                    return Some((b, index as usize));
                }
            }
        } else {
            for (b, bucket) in self.buckets.iter().enumerate() {
                if (index as usize) < bucket.len() {
                    return Some((b, index as usize));
                }
                index -= bucket.len() as isize;
            }
        }
        None
    }
    /// `index` 番目の要素を参照します。負数は末尾から数え、範囲外なら `None` を返します。
    pub fn get(&self, index: isize) -> Option<&T> {
        let (b, i) = self.locate(index)?;
        Some(&self.buckets[b][i])
    }
    /// `index` 番目の可変参照を返します。負数は末尾から数え、範囲外なら `None` です。
    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        let (b, i) = self.locate(index)?;
        Some(&mut self.buckets[b][i])
    }
    fn insert_at(&mut self, b: usize, i: usize, value: T) {
        self.buckets[b].insert(i, value);
        self.len += 1;
        if self.buckets[b].len() > self.buckets.len() * 24 {
            let mid = self.buckets[b].len() / 2;
            let right = self.buckets[b].split_off(mid);
            self.buckets.insert(b + 1, right);
        }
    }
    /// `index` 番目の直前に `value` を挿入します。`len()` の位置なら末尾に追加します。
    /// 負数は末尾基準で、空の場合は `0` と `-1` のみ有効です。
    /// 範囲外なら変更せず `Err(value)` を返します。
    pub fn insert(&mut self, index: isize, value: T) -> Result<(), T> {
        if self.is_empty() {
            if index != 0 && index != -1 {
                return Err(value);
            }
            self.append(value);
        } else if index >= 0 && index as usize == self.len {
            self.append(value);
        } else if let Some((b, i)) = self.locate(index) {
            self.insert_at(b, i, value);
        } else {
            return Err(value);
        }
        Ok(())
    }
    /// `value` の所有権を受け取り、末尾に追加します。
    pub fn append(&mut self, value: T) {
        if self.is_empty() {
            self.buckets.push(vec![value]);
            self.len = 1;
        } else {
            let b = self.buckets.len() - 1;
            self.insert_at(b, self.buckets[b].len(), value);
        }
    }
    fn remove_at(&mut self, b: usize, i: usize) -> T {
        let value = self.buckets[b].remove(i);
        self.len -= 1;
        if self.buckets[b].is_empty() {
            self.buckets.remove(b);
        }
        value
    }
    /// `index` 番目を削除し、値の所有権を返します。負数は末尾基準です。
    /// 範囲外なら変更せず `None` を返します。
    pub fn pop(&mut self, index: isize) -> Option<T> {
        let (b, i) = self.locate(index)?;
        Some(self.remove_at(b, i))
    }
    /// 末尾の要素を削除して返します。空なら `None` です。`pop(-1)` と同じです。
    pub fn pop_last(&mut self) -> Option<T> {
        self.pop(-1)
    }
    /// 要素の並びをその場で反転します。O(N)。
    pub fn reverse(&mut self) {
        self.buckets.reverse();
        for bucket in &mut self.buckets {
            bucket.reverse();
        }
    }
}
impl<T: PartialEq> BucketList<T> {
    /// 指定した値が存在すれば `true` を返します。
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|x| x == value)
    }
    /// 指定した値の出現回数を返します。存在しなければ `0` です。
    pub fn count(&self, value: &T) -> usize {
        self.iter().filter(|x| *x == value).count()
    }
    /// 指定した値が最初に現れる添字を返します。存在しなければ `None` です。O(N)。
    pub fn index(&self, value: &T) -> Option<usize> {
        self.iter().position(|x| x == value)
    }
    /// 指定した値の最初の出現を削除し、その所有権を返します。存在しなければ `None` です。O(N)。
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let index = self.index(value)?;
        self.pop(index as isize)
    }
}
impl<T: PartialEq> PartialEq for BucketList<T> {
    /// バケット構成によらず、要素の並びが等しいか比較します。
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}
impl<T: Eq> Eq for BucketList<T> {}
impl<T> FromIterator<T> for BucketList<T> {
    /// 入力の順序を保って O(N) で構築します。
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let values: Vec<T> = iter.into_iter().collect();
        let len = values.len();
        let count = ((len as f64 / 16.0).sqrt().ceil() as usize).max(1);
        let mut iter = values.into_iter();
        let buckets = if len == 0 {
            Vec::new()
        } else {
            (0..count)
                .map(|i| {
                    iter.by_ref()
                        .take(len * (i + 1) / count - len * i / count)
                        .collect()
                })
                .collect()
        };
        Self { buckets, len }
    }
}
impl<T> Extend<T> for BucketList<T> {
    /// 入力の各値を順に末尾へ追加します。
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.append(x);
        }
    }
}
impl<'a, T> IntoIterator for &'a BucketList<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, Vec<T>>>;
    /// コンテナを消費せず、要素参照の双方向イテレータを返します。
    fn into_iter(self) -> Self::IntoIter {
        self.buckets.iter().flatten()
    }
}

/// バケット分割で管理する順序付き集合の共通実装。通常は型エイリアスを利用してください。
#[derive(Clone, Debug)]
pub struct SortedCollection<T, const MULTI: bool> {
    data: BucketList<T>,
}
/// 重複を除く順序付き集合。構築・更新・検索には `T: Ord` が必要です。
pub type SortedSet<T> = SortedCollection<T, false>;
/// 重複を許す順序付き多重集合。削除は 1 個ずつ行います。
pub type SortedMultiset<T> = SortedCollection<T, true>;

impl<T: Ord, const M: bool> Default for SortedCollection<T, M> {
    /// 空のコンテナを作ります。
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord, const M: bool> SortedCollection<T, M> {
    /// 空のコンテナを作ります。O(1)。
    pub fn new() -> Self {
        Self {
            data: BucketList::new(),
        }
    }
    /// 格納している要素数を返します。重複も数えます。O(1)。
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// 要素が存在しない場合に `true` を返します。O(1)。
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// 内部バケットの読み取り専用ビューを返します。空のバケットは含みません。O(1)。
    pub fn buckets(&self) -> &[Vec<T>] {
        self.data.buckets()
    }
    /// 要素への読み取り専用の双方向イテレータを返します。`rev()` で逆順に走査できます。 比較順序に沿って走査します。
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.data.iter()
    }
    /// すべての要素を削除して空にします。
    pub fn clear(&mut self) {
        self.data.clear();
    }
    /// `index` 番目の要素を参照します。負数は末尾から数え、範囲外なら `None` を返します。
    pub fn get(&self, index: isize) -> Option<&T> {
        self.data.get(index)
    }
    /// `index` 番目を削除し、値の所有権を返します。負数は末尾基準です。
    /// 範囲外なら変更せず `None` を返します。
    pub fn pop(&mut self, index: isize) -> Option<T> {
        self.data.pop(index)
    }
    /// 末尾の要素を削除して返します。空なら `None` です。`pop(-1)` と同じです。
    pub fn pop_last(&mut self) -> Option<T> {
        self.pop(-1)
    }
    fn position(&self, x: &T) -> Option<(usize, usize)> {
        if self.is_empty() {
            return None;
        }
        let b = self
            .data
            .buckets
            .iter()
            .position(|a| a.last().unwrap() >= x)
            .unwrap_or(self.data.buckets.len() - 1);
        Some((b, self.data.buckets[b].partition_point(|y| y < x)))
    }
    /// 指定した値が存在すれば `true` を返します。
    pub fn contains(&self, x: &T) -> bool {
        self.position(x)
            .is_some_and(|(b, i)| self.data.buckets[b].get(i) == Some(x))
    }
    /// `x` を比較順序に従って挿入します。集合では既存なら `false`、新規なら `true`。
    /// 多重集合では常に 1 個追加して `true` を返します。
    pub fn add(&mut self, x: T) -> bool {
        if let Some((b, i)) = self.position(&x) {
            if !M && self.data.buckets[b].get(i) == Some(&x) {
                return false;
            }
            self.data.insert_at(b, i, x);
        } else {
            self.data.append(x);
        }
        true
    }
    /// `x` を 1 個だけ削除します。削除できれば `true`、存在しなければ `false` です。
    pub fn discard(&mut self, x: &T) -> bool {
        if let Some((b, i)) = self.position(x) {
            if self.data.buckets[b].get(i) == Some(x) {
                self.data.remove_at(b, i);
                return true;
            }
        }
        false
    }
    /// `x` 未満の最大要素への参照を返します。該当要素がなければ `None` です。
    pub fn lt(&self, x: &T) -> Option<&T> {
        for a in self.data.buckets.iter().rev() {
            if &a[0] < x {
                return a.get(a.partition_point(|y| y < x) - 1);
            }
        }
        None
    }
    /// `x` 以下の最大要素への参照を返します。該当要素がなければ `None` です。
    pub fn le(&self, x: &T) -> Option<&T> {
        for a in self.data.buckets.iter().rev() {
            if &a[0] <= x {
                return a.get(a.partition_point(|y| y <= x) - 1);
            }
        }
        None
    }
    /// `x` より大きい最小要素への参照を返します。該当要素がなければ `None` です。
    pub fn gt(&self, x: &T) -> Option<&T> {
        for a in &self.data.buckets {
            if a.last().unwrap() > x {
                return a.get(a.partition_point(|y| y <= x));
            }
        }
        None
    }
    /// `x` 以上の最小要素への参照を返します。該当要素がなければ `None` です。
    pub fn ge(&self, x: &T) -> Option<&T> {
        for a in &self.data.buckets {
            if a.last().unwrap() >= x {
                return a.get(a.partition_point(|y| y < x));
            }
        }
        None
    }
    /// `x` 未満の要素数を返します。`x` 自身が存在しなくても利用できます。
    pub fn index(&self, x: &T) -> usize {
        let mut n = 0;
        for a in &self.data.buckets {
            if a.last().unwrap() >= x {
                return n + a.partition_point(|y| y < x);
            }
            n += a.len();
        }
        n
    }
    /// `x` 以下の要素数を返します。`x` 自身が存在しなくても利用できます。
    pub fn index_right(&self, x: &T) -> usize {
        let mut n = 0;
        for a in &self.data.buckets {
            if a.last().unwrap() > x {
                return n + a.partition_point(|y| y <= x);
            }
            n += a.len();
        }
        n
    }
    /// 指定した値の出現回数を返します。存在しなければ `0` です。
    pub fn count(&self, x: &T) -> usize {
        self.index_right(x) - self.index(x)
    }
}
impl<T: Ord, const M: bool> FromIterator<T> for SortedCollection<T, M> {
    /// 入力を比較順序で構築し、集合の場合は重複を除きます。
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut values: Vec<T> = iter.into_iter().collect();
        if values.windows(2).any(|w| w[0] > w[1]) {
            values.sort_unstable();
        }
        if !M {
            values.dedup();
        }
        Self {
            data: values.into_iter().collect(),
        }
    }
}
impl<T: Ord, const M: bool> Extend<T> for SortedCollection<T, M> {
    /// 各値を順に追加します。集合では重複を追加しません。
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.add(x);
        }
    }
}
impl<'a, T, const M: bool> IntoIterator for &'a SortedCollection<T, M> {
    type Item = &'a T;
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, Vec<T>>>;
    /// コンテナを消費せず、要素参照の双方向イテレータを返します。
    fn into_iter(self) -> Self::IntoIter {
        (&self.data).into_iter()
    }
}
impl<T: PartialEq, const M: bool> PartialEq for SortedCollection<T, M> {
    /// バケット構成によらず、要素の並びが等しいか比較します。
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl<T: Eq, const M: bool> Eq for SortedCollection<T, M> {}
impl<T: Ord, const M: bool> std::ops::Index<isize> for SortedCollection<T, M> {
    type Output = T;
    /// 指定添字の参照を返します。負数は末尾基準です。
    ///
    /// # Panics
    /// 範囲外の添字を渡すと panic します。
    fn index(&self, index: isize) -> &T {
        self.get(index)
            .expect("SortedCollection index out of bounds")
    }
}
impl<T> std::ops::Index<isize> for BucketList<T> {
    type Output = T;
    /// 指定添字の参照を返します。負数は末尾基準です。
    ///
    /// # Panics
    /// 範囲外の添字を渡すと panic します。
    fn index(&self, index: isize) -> &T {
        self.get(index).expect("BucketList index out of bounds")
    }
}
impl<T> std::ops::IndexMut<isize> for BucketList<T> {
    /// 指定添字の参照を返します。負数は末尾基準です。
    ///
    /// # Panics
    /// 範囲外の添字を渡すと panic します。
    fn index_mut(&mut self, index: isize) -> &mut T {
        self.get_mut(index).expect("BucketList index out of bounds")
    }
}
