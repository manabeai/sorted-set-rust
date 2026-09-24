// Standalone BucketList; standard library only. Unlicense.
// Port of https://github.com/tatyam-prime/SortedSet

/// 挿入順を保持するバケット方式のリスト。
///
/// # Examples
///
/// ```
/// use sorted_set::BucketList;
/// let s: BucketList<_> = [3, 1, 3].into_iter().collect();
/// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [3, 1, 3]);
/// ```
#[derive(Clone, Debug)]
pub struct BucketList<T> {
    buckets: Vec<Vec<T>>,
    len: usize,
}

impl<T> Default for BucketList<T> {
    /// 空のコンテナを作ります。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s = BucketList::<i32>::default();
    /// assert!(s.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BucketList<T> {
    /// 空のコンテナを作ります。O(1)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s = BucketList::<i32>::new();
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
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
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
    /// use sorted_set::BucketList;
    /// let mut s = BucketList::<i32>::new();
    /// assert!(s.is_empty());
    /// s.append(1);
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
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// assert!(s.buckets().iter().all(|bucket| !bucket.is_empty()));
    /// assert_eq!(s.buckets().iter().flatten().copied().collect::<Vec<_>>(), [1, 3, 5]);
    /// ```
    pub fn buckets(&self) -> &[Vec<T>] {
        &self.buckets
    }
    /// 要素への読み取り専用の双方向イテレータを返します。`rev()` で逆順に走査できます。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
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
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1, 3, 5].into_iter().collect();
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
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.get(0), Some(&1));
    /// assert_eq!(s.get(-1), Some(&5));
    /// assert_eq!(s.get(3), None);
    /// ```
    pub fn get(&self, index: isize) -> Option<&T> {
        let (b, i) = self.locate(index)?;
        Some(&self.buckets[b][i])
    }
    /// `index` 番目の可変参照を返します。負数は末尾から数え、範囲外なら `None` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// *s.get_mut(-1).unwrap() = 9;
    /// assert_eq!(s.get(-1), Some(&9));
    /// assert_eq!(s.get_mut(3), None);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s.insert(-1, 4), Ok(()));
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3, 4, 5]);
    /// assert_eq!(s.insert(99, 7), Err(7));
    /// assert_eq!(s.len(), 4);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// s.append(2);
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3, 5, 2]);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1, 3, 5].into_iter().collect();
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
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [5].into_iter().collect();
    /// assert_eq!(s.pop_last(), Some(5));
    /// assert_eq!(s.pop_last(), None);
    /// ```
    pub fn pop_last(&mut self) -> Option<T> {
        self.pop(-1)
    }
    /// 要素の並びをその場で反転します。O(N)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// s.reverse();
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [5, 3, 1]);
    /// ```
    pub fn reverse(&mut self) {
        self.buckets.reverse();
        for bucket in &mut self.buckets {
            bucket.reverse();
        }
    }
}
impl<T: PartialEq> BucketList<T> {
    /// 指定した値が存在すれば `true` を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// assert!(s.contains(&3));
    /// assert!(!s.contains(&2));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|x| x == value)
    }
    /// 指定した値の出現回数を返します。存在しなければ `0` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 3].into_iter().collect();
    /// assert_eq!(s.count(&3), 2);
    /// assert_eq!(s.count(&2), 0);
    /// ```
    pub fn count(&self, value: &T) -> usize {
        self.iter().filter(|x| *x == value).count()
    }
    /// 指定した値が最初に現れる添字を返します。存在しなければ `None` です。O(N)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [3, 1, 3].into_iter().collect();
    /// assert_eq!(s.index(&3), Some(0));
    /// assert_eq!(s.index(&9), None);
    /// ```
    pub fn index(&self, value: &T) -> Option<usize> {
        self.iter().position(|x| x == value)
    }
    /// 指定した値の最初の出現を削除し、その所有権を返します。存在しなければ `None` です。O(N)。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [3, 1, 3].into_iter().collect();
    /// assert_eq!(s.remove(&3), Some(3));
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3]);
    /// assert_eq!(s.remove(&9), None);
    /// ```
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let index = self.index(value)?;
        self.pop(index as isize)
    }
}
impl<T: PartialEq> PartialEq for BucketList<T> {
    /// バケット構成によらず、要素の並びが等しいか比較します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// let copy = s.clone();
    /// assert_eq!(s, copy);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}
impl<T: Eq> Eq for BucketList<T> {}
impl<T> std::iter::FromIterator<T> for BucketList<T> {
    /// 入力の順序を保って O(N) で構築します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [3, 1, 3].into_iter().collect();
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [3, 1, 3]);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1].into_iter().collect();
    /// s.extend([3, 3]);
    /// assert_eq!(s.iter().copied().collect::<Vec<_>>(), [1, 3, 3]);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// let values: Vec<_> = (&s).into_iter().copied().collect();
    /// assert_eq!(values, [1, 3, 5]);
    /// assert_eq!(s.len(), 3);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.buckets.iter().flatten()
    }
}

impl<T> std::ops::Index<isize> for BucketList<T> {
    type Output = T;
    /// 指定添字の参照を返します。負数は末尾基準です。
    ///
    /// # Panics
    /// 範囲外の添字を渡すと panic します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// assert_eq!(s[0], 1);
    /// assert_eq!(s[-1], 5);
    /// ```
    ///
    /// ```should_panic
    /// use sorted_set::BucketList;
    /// let s = BucketList::<i32>::new();
    /// let _ = s[0]; // 空のコンテナへの添字アクセスは panic します。
    /// ```
    fn index(&self, index: isize) -> &T {
        self.get(index).expect("BucketList index out of bounds")
    }
}
impl<T> std::ops::IndexMut<isize> for BucketList<T> {
    /// 指定添字の参照を返します。負数は末尾基準です。
    ///
    /// # Panics
    /// 範囲外の添字を渡すと panic します。
    ///
    /// # Examples
    ///
    /// ```
    /// use sorted_set::BucketList;
    /// let mut s: BucketList<_> = [1, 3, 5].into_iter().collect();
    /// s[-1] = 9;
    /// assert_eq!(s[-1], 9);
    /// ```
    ///
    /// ```should_panic
    /// use sorted_set::BucketList;
    /// let s = BucketList::<i32>::new();
    /// let _ = s[0]; // 空のコンテナへの添字アクセスは panic します。
    /// ```
    fn index_mut(&mut self, index: isize) -> &mut T {
        self.get_mut(index).expect("BucketList index out of bounds")
    }
}
