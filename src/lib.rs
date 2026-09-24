//! Bucket-based containers ported from <https://github.com/tatyam-prime/SortedSet>.
//! Each implementation file is self-contained and can also be copied into a submission.
//!
//! ```
//! use sorted_set::SortedSet;
//! let mut s: SortedSet<_> = [3, 1, 2, 1].into_iter().collect();
//! assert_eq!(s.index(&3), 2);
//! assert_eq!(s.lt(&3), Some(&2));
//! assert_eq!(s.pop_last(), Some(3));
//! ```
#![deny(missing_docs)]

#[path = "bucket_list.rs"]
mod bucket_list_impl;
#[path = "sorted_multiset.rs"]
mod sorted_multiset_impl;
#[path = "sorted_set.rs"]
mod sorted_set_impl;

pub use bucket_list_impl::BucketList;
pub use sorted_multiset_impl::SortedMultiset;
pub use sorted_set_impl::SortedSet;
