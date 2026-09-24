use sorted_set::{BucketList, SortedMultiset, SortedSet};

fn main() {
    let mut set: SortedSet<_> = [3, 1, 4, 1, 5].into_iter().collect();
    assert!(!set.add(3));
    assert!(set.add(2));
    assert_eq!(set.lt(&3), Some(&2));
    assert_eq!(set.ge(&3), Some(&3));
    assert_eq!(set.index(&3), 2);
    assert_eq!(set[-1], 5);
    assert_eq!(set.pop_last(), Some(5));
    println!("{:?}", set.iter().collect::<Vec<_>>());

    let mut multi: SortedMultiset<_> = [2, 2, 1].into_iter().collect();
    multi.discard(&2);
    assert_eq!(multi.count(&2), 1);

    let mut list: BucketList<_> = [3, 1, 2].into_iter().collect();
    list.insert(-1, 9).unwrap();
    list.reverse();
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), [2, 9, 1, 3]);
}
