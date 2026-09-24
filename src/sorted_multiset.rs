// Standalone SortedMultiset; standard library only. Unlicense.
// Port of https://github.com/tatyam-prime/SortedSet

/// 重複を許す順序付き多重集合。
///
/// # Examples
///
/// ```
/// use sorted_set::SortedMultiset;
/// let s: SortedMultiset<_> = [3, 1, 3].into_iter().collect();
/// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3, 3]);
/// ```
#[derive(Clone, Debug)]
pub struct SortedMultiset<T> {
    buckets: Vec<Vec<T>>,
    len: usize,
}

impl<T: Ord> SortedMultiset<T> {
    /// 空のコンテナを作ります。O(1)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s = SortedMultiset::<i32>::new();
    /// assert!(s.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
    }

    /// 格納している要素数を返します。重複も数えます。O(1)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// 要素が存在しない場合に `true` を返します。O(1)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let mut s = SortedMultiset::<i32>::new();
    /// assert!(s.is_empty());
    /// s.add(1);
    /// assert!(!s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 内部バケットの読み取り専用ビューを返します。空のバケットは含みません。O(1)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert!(s.buckets().iter().all(|bucket| !bucket.is_empty()));
    /// assert_eq!(s.buckets().iter().flatten().copied().collect::<Vec<_>>(), [1, 3, 5]);
    /// ```
    pub fn buckets(&self) -> &[Vec<T>] {
        &self.buckets
    }

    /// 比較順序に沿った読み取り専用の双方向イテレータを返します。`rev()` で逆順に走査できます。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3, 5]);
    /// assert_eq!(s.iter().rev().copied().collect::<Vec<_>>(), [5, 3, 1]);
    /// ```
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.buckets.iter().flatten()
    }

    /// すべての要素を削除して空にします。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let mut s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// s.clear();
    /// assert!(s.is_empty());
    /// assert!(s.buckets().is_empty());
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.get(0), Some(&1));
    /// assert_eq!(s.get(-1), Some(&5));
    /// assert_eq!(s.get(3), None);
    /// ```
    pub fn get(&self, index: isize) -> Option<&T> {
        let (b, i) = self.locate(index)?;
        Some(&self.buckets[b][i])
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

    /// `value` の所有権を受け取り、末尾に追加します。
    fn append(&mut self, value: T) {
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let mut s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.pop(-1), Some(5));
    /// assert_eq!(s.pop(0), Some(1));
    /// assert_eq!(s.pop(9), None);
    /// assert_eq!(s.len(), 1);
    /// ```
    pub fn pop(&mut self, index: isize) -> Option<T> {
        let (b, i) = self.locate(index)?;
        Some(self.remove_at(b, i))
    }

    /// 末尾の要素を削除して返します。空なら `None` です。`pop(-1)` と同じです。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let mut s: SortedMultiset<_> = [5].into_iter().collect();
    /// assert_eq!(s.pop_last(), Some(5));
    /// assert_eq!(s.pop_last(), None);
    /// ```
    pub fn pop_last(&mut self) -> Option<T> {
        self.pop(-1)
    }

    fn position(&self, x: &T) -> Option<(usize, usize)> {
        if self.is_empty() {
            return None;
        }
        let b = self
            .buckets
            .iter()
            .position(|a| a.last().unwrap() >= x)
            .unwrap_or(self.buckets.len() - 1);
        Some((b, self.buckets[b].partition_point(|y| y < x)))
    }

    /// 指定した値が存在すれば `true` を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert!(s.contains(&3));
    /// assert!(!s.contains(&2));
    /// ```
    pub fn contains(&self, x: &T) -> bool {
        self.position(x)
            .is_some_and(|(b, i)| self.buckets[b].get(i) == Some(x))
    }

    /// `x` を比較順序に従って 1 個追加し、常に `true` を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let mut s = SortedMultiset::new();
    /// assert!(s.add(3));
    /// assert!(s.add(3));
    /// assert_eq!(s.count(&3), 2);
    /// ```
    pub fn add(&mut self, x: T) -> bool {
        if let Some((b, i)) = self.position(&x) {
            self.insert_at(b, i, x);
        } else {
            self.append(x);
        }
        true
    }

    /// `x` を 1 個だけ削除します。削除できれば `true`、存在しなければ `false` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let mut s: SortedMultiset<_> = [3, 3].into_iter().collect();
    /// assert!(s.discard(&3));
    /// assert_eq!(s.count(&3), 1);
    /// assert!(!s.discard(&9));
    /// ```
    pub fn discard(&mut self, x: &T) -> bool {
        if let Some((b, i)) = self.position(x) {
            if self.buckets[b].get(i) == Some(x) {
                self.remove_at(b, i);
                return true;
            }
        }
        false
    }

    /// `x` 未満の最大要素への参照を返します。該当要素がなければ `None` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.lt(&3), Some(&1));
    /// assert_eq!(s.lt(&1), None);
    /// ```
    pub fn lt(&self, x: &T) -> Option<&T> {
        for a in self.buckets.iter().rev() {
            if &a[0] < x {
                return a.get(a.partition_point(|y| y < x) - 1);
            }
        }
        None
    }

    /// `x` 以下の最大要素への参照を返します。該当要素がなければ `None` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.le(&3), Some(&3));
    /// assert_eq!(s.le(&0), None);
    /// ```
    pub fn le(&self, x: &T) -> Option<&T> {
        for a in self.buckets.iter().rev() {
            if &a[0] <= x {
                return a.get(a.partition_point(|y| y <= x) - 1);
            }
        }
        None
    }

    /// `x` より大きい最小要素への参照を返します。該当要素がなければ `None` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.gt(&3), Some(&5));
    /// assert_eq!(s.gt(&5), None);
    /// ```
    pub fn gt(&self, x: &T) -> Option<&T> {
        for a in &self.buckets {
            if a.last().unwrap() > x {
                return a.get(a.partition_point(|y| y <= x));
            }
        }
        None
    }

    /// `x` 以上の最小要素への参照を返します。該当要素がなければ `None` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.ge(&3), Some(&3));
    /// assert_eq!(s.ge(&6), None);
    /// ```
    pub fn ge(&self, x: &T) -> Option<&T> {
        for a in &self.buckets {
            if a.last().unwrap() >= x {
                return a.get(a.partition_point(|y| y < x));
            }
        }
        None
    }

    /// `x` 未満の要素数を返します。`x` 自身が存在しなくても利用できます。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.index(&3), 1);
    /// assert_eq!(s.index(&4), 2);
    /// ```
    pub fn index(&self, x: &T) -> usize {
        let mut n = 0;
        for a in &self.buckets {
            if a.last().unwrap() >= x {
                return n + a.partition_point(|y| y < x);
            }
            n += a.len();
        }
        n
    }

    /// `x` 以下の要素数を返します。`x` 自身が存在しなくても利用できます。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.index_right(&3), 2);
    /// assert_eq!(s.index_right(&4), 2);
    /// ```
    pub fn index_right(&self, x: &T) -> usize {
        let mut n = 0;
        for a in &self.buckets {
            if a.last().unwrap() > x {
                return n + a.partition_point(|y| y <= x);
            }
            n += a.len();
        }
        n
    }

    /// 指定した値の出現回数を返します。存在しなければ `0` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 3].into_iter().collect();
    /// assert_eq!(s.count(&3), 2);
    /// assert_eq!(s.count(&2), 0);
    /// ```
    pub fn count(&self, x: &T) -> usize {
        self.index_right(x) - self.index(x)
    }
}

impl<T: Ord> Default for SortedMultiset<T> {
    /// 空のコンテナを作ります。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s = SortedMultiset::<i32>::default();
    /// assert!(s.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> std::iter::FromIterator<T> for SortedMultiset<T> {
    /// 入力を比較順序で構築します。ソート済みなら O(N)、それ以外は O(N log N)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [3, 1, 3].into_iter().collect();
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3, 3]);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut values: Vec<T> = iter.into_iter().collect();

        if values.windows(2).any(|w| w[0] > w[1]) {
            values.sort_unstable();
        }
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

impl<T: Ord> Extend<T> for SortedMultiset<T> {
    /// 各値を順に追加します。集合では重複を追加しません。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let mut s: SortedMultiset<_> = [1].into_iter().collect();
    /// s.extend([3, 3]);
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3, 3]);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.add(x);
        }
    }
}

impl<T: PartialEq> PartialEq for SortedMultiset<T> {
    /// バケット構成によらず、要素の並びが等しいか比較します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// let copy = s.clone();
    /// assert_eq!(s, copy);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
            && self
                .buckets
                .iter()
                .flatten()
                .eq(other.buckets.iter().flatten())
    }
}

impl<T: Eq> Eq for SortedMultiset<T> {}

impl<'a, T> IntoIterator for &'a SortedMultiset<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, Vec<T>>>;
    /// コンテナを消費せず、要素参照の双方向イテレータを返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// let values: Vec<_> = (&s).into_iter().copied().collect();
    /// assert_eq!(values, [1, 3, 5]);
    /// assert_eq!(s.len(), 3);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.buckets.iter().flatten()
    }
}

impl<T: Ord> std::ops::Index<isize> for SortedMultiset<T> {
    type Output = T;
    /// 指定添字の参照を返します。負数は末尾基準です。
    ///
    /// # Panics
    /// 範囲外の添字を渡すと panic します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::SortedMultiset;
    /// let s: SortedMultiset<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[-1], 5);
    /// ```
    ///
    /// ```should_panic
    /// use sorted_set::SortedMultiset;
    /// let s = SortedMultiset::<i32>::new();
    /// let _ = s[0]; // 空のコンテナへの添字アクセスは panic します。
    /// ```
    fn index(&self, index: isize) -> &T {
        self.get(index).expect("SortedMultiset index out of bounds")
    }
}
