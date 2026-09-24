//! Bucket-based containers ported from <https://github.com/tatyam-prime/SortedSet>.
//! Negative indices count from the end. Missing values and invalid indices return `None`.
//! Sorted containers deliberately expose only immutable element references.

/// An unsorted sequence backed by split-on-growth buckets.
#[derive(Clone, Debug)]
pub struct BucketList<T> {
    buckets: Vec<Vec<T>>,
    len: usize,
}

impl<T> Default for BucketList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BucketList<T> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn buckets(&self) -> &[Vec<T>] {
        &self.buckets
    }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.buckets.iter().flatten()
    }
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
    pub fn get(&self, index: isize) -> Option<&T> {
        let (b, i) = self.locate(index)?;
        Some(&self.buckets[b][i])
    }
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
    /// Returns the uninserted value for an invalid index (unlike Python, does not panic).
    /// As upstream, inserting into an empty list accepts only 0 and -1.
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
    pub fn pop(&mut self, index: isize) -> Option<T> {
        let (b, i) = self.locate(index)?;
        Some(self.remove_at(b, i))
    }
    pub fn pop_last(&mut self) -> Option<T> {
        self.pop(-1)
    }
    pub fn reverse(&mut self) {
        self.buckets.reverse();
        for bucket in &mut self.buckets {
            bucket.reverse();
        }
    }
}
impl<T: PartialEq> BucketList<T> {
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|x| x == value)
    }
    pub fn count(&self, value: &T) -> usize {
        self.iter().filter(|x| *x == value).count()
    }
    pub fn index(&self, value: &T) -> Option<usize> {
        self.iter().position(|x| x == value)
    }
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let index = self.index(value)?;
        self.pop(index as isize)
    }
}
impl<T: PartialEq> PartialEq for BucketList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}
impl<T: Eq> Eq for BucketList<T> {}
impl<T> FromIterator<T> for BucketList<T> {
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
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.append(x);
        }
    }
}
impl<'a, T> IntoIterator for &'a BucketList<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, Vec<T>>>;
    fn into_iter(self) -> Self::IntoIter {
        self.buckets.iter().flatten()
    }
}

/// Shared implementation; prefer the `SortedSet` and `SortedMultiset` aliases.
#[derive(Clone, Debug)]
pub struct SortedCollection<T, const MULTI: bool> {
    data: BucketList<T>,
}
pub type SortedSet<T> = SortedCollection<T, false>;
pub type SortedMultiset<T> = SortedCollection<T, true>;

impl<T: Ord, const M: bool> Default for SortedCollection<T, M> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Ord, const M: bool> SortedCollection<T, M> {
    pub fn new() -> Self {
        Self {
            data: BucketList::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn buckets(&self) -> &[Vec<T>] {
        self.data.buckets()
    }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.data.iter()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn get(&self, index: isize) -> Option<&T> {
        self.data.get(index)
    }
    pub fn pop(&mut self, index: isize) -> Option<T> {
        self.data.pop(index)
    }
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
    pub fn contains(&self, x: &T) -> bool {
        self.position(x)
            .is_some_and(|(b, i)| self.data.buckets[b].get(i) == Some(x))
    }
    /// Inserts one value. Returns false only for an already-present set value.
    /// For a multiset this always returns true (Python's version returns None).
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
    /// Removes exactly one occurrence, including for multisets.
    pub fn discard(&mut self, x: &T) -> bool {
        if let Some((b, i)) = self.position(x) {
            if self.data.buckets[b].get(i) == Some(x) {
                self.data.remove_at(b, i);
                return true;
            }
        }
        false
    }
    pub fn lt(&self, x: &T) -> Option<&T> {
        for a in self.data.buckets.iter().rev() {
            if &a[0] < x {
                return a.get(a.partition_point(|y| y < x) - 1);
            }
        }
        None
    }
    pub fn le(&self, x: &T) -> Option<&T> {
        for a in self.data.buckets.iter().rev() {
            if &a[0] <= x {
                return a.get(a.partition_point(|y| y <= x) - 1);
            }
        }
        None
    }
    pub fn gt(&self, x: &T) -> Option<&T> {
        for a in &self.data.buckets {
            if a.last().unwrap() > x {
                return a.get(a.partition_point(|y| y <= x));
            }
        }
        None
    }
    pub fn ge(&self, x: &T) -> Option<&T> {
        for a in &self.data.buckets {
            if a.last().unwrap() >= x {
                return a.get(a.partition_point(|y| y < x));
            }
        }
        None
    }
    /// Number of elements strictly less than x (not a membership lookup).
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
    pub fn count(&self, x: &T) -> usize {
        self.index_right(x) - self.index(x)
    }
}
impl<T: Ord, const M: bool> FromIterator<T> for SortedCollection<T, M> {
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
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.add(x);
        }
    }
}
impl<'a, T, const M: bool> IntoIterator for &'a SortedCollection<T, M> {
    type Item = &'a T;
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, Vec<T>>>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.data).into_iter()
    }
}
impl<T: PartialEq, const M: bool> PartialEq for SortedCollection<T, M> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl<T: Eq, const M: bool> Eq for SortedCollection<T, M> {}
impl<T: Ord, const M: bool> std::ops::Index<isize> for SortedCollection<T, M> {
    type Output = T;
    fn index(&self, index: isize) -> &T {
        self.get(index)
            .expect("SortedCollection index out of bounds")
    }
}
impl<T> std::ops::Index<isize> for BucketList<T> {
    type Output = T;
    fn index(&self, index: isize) -> &T {
        self.get(index).expect("BucketList index out of bounds")
    }
}
impl<T> std::ops::IndexMut<isize> for BucketList<T> {
    fn index_mut(&mut self, index: isize) -> &mut T {
        self.get_mut(index).expect("BucketList index out of bounds")
    }
}
