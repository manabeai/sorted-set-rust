use sorted_set::{BucketList, SortedMultiset, SortedSet};

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

macro_rules! sorted_tests {
    ($module:ident, $kind:ident, $multi:expr) => {
        mod $module {
            use super::*;
            fn check(s: &$kind<i32>, v: &[i32], x: i32) {
                assert_eq!(s.len(), v.len());
                assert_eq!(s.iter().copied().collect::<Vec<_>>(), v);
                assert_eq!(
                    s.iter().rev().copied().collect::<Vec<_>>(),
                    v.iter().rev().copied().collect::<Vec<_>>()
                );
                assert!(s.buckets().iter().all(|a| !a.is_empty()));
                let l = v.partition_point(|y| *y < x);
                let r = v.partition_point(|y| *y <= x);
                assert_eq!(s.index(&x), l);
                assert_eq!(s.index_right(&x), r);
                assert_eq!(s.count(&x), r - l);
                assert_eq!(s.contains(&x), l != r);
                assert_eq!(s.lt(&x), l.checked_sub(1).and_then(|i| v.get(i)));
                assert_eq!(s.le(&x), r.checked_sub(1).and_then(|i| v.get(i)));
                assert_eq!(s.ge(&x), v.get(l));
                assert_eq!(s.gt(&x), v.get(r));
                assert_eq!(s.get(-1), v.last());
                assert_eq!(s.get(0), v.first());
                assert_eq!(s.get(v.len() as isize), None);
                assert_eq!(s.get(-(v.len() as isize) - 1), None);
                assert_eq!(s.get(isize::MIN), None);
                assert_eq!(s.get(isize::MAX), None);
            }

            #[test]
            fn randomized() {
                let mut s = $kind::<i32>::new();
                let mut v = Vec::new();
                let mut seed = 812;
                for _ in 0..15000 {
                    let x = (next(&mut seed) % 401) as i32 - 200;
                    match next(&mut seed) % 5 {
                        0..=2 => {
                            let i = v.partition_point(|y| *y < x);
                            let added = $multi || v.get(i) != Some(&x);
                            assert_eq!(s.add(x), added);
                            if added {
                                v.insert(i, x);
                            }
                        }
                        3 => {
                            let i = v.partition_point(|y| *y < x);
                            let found = v.get(i) == Some(&x);
                            assert_eq!(s.discard(&x), found);
                            if found {
                                v.remove(i);
                            }
                        }
                        _ if !v.is_empty() => {
                            let i = next(&mut seed) as usize % v.len();
                            let signed = if next(&mut seed) & 1 == 0 {
                                i as isize
                            } else {
                                i as isize - v.len() as isize
                            };
                            assert_eq!(s.get(signed), Some(&v[i]));
                            assert_eq!(s.pop(signed), Some(v.remove(i)));
                        }
                        _ => {
                            assert_eq!(s.pop_last(), None);
                        }
                    }
                    check(&s, &v, x);
                }
                while !v.is_empty() {
                    assert_eq!(s.pop(0), Some(v.remove(0)));
                }
                assert!(s.buckets().is_empty());
                assert!(s.add(7));
                s.clear();
                assert!(s.is_empty());
            }
        }
    };
}
sorted_tests!(set_tests, SortedSet, false);
sorted_tests!(multiset_tests, SortedMultiset, true);

#[test]
fn construction_splits_equality_and_non_clone() {
    let a: SortedSet<_> = [3, 1, 3, 2].into_iter().collect();
    assert_eq!(a, [1, 2, 3].into_iter().collect());
    let mut a = SortedSet::new();
    for x in 0..10000 {
        a.add(x);
    }
    assert_eq!(a, (0..10000).collect());
    for x in 0..10000 {
        assert!(a.discard(&x));
    }
    let mut m: SortedMultiset<_> = vec![4; 5000].into_iter().collect();
    assert_eq!(m.count(&4), 5000);
    assert_eq!(m.lt(&4), None);
    assert_eq!(m.gt(&4), None);
    for _ in 0..5000 {
        assert!(m.discard(&4));
    }
    assert!(!m.discard(&4));
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Value(i32);
    let mut s = SortedSet::new();
    s.add(Value(2));
    s.add(Value(1));
    assert_eq!(s.pop_last(), Some(Value(2)));
}

#[test]
fn bucket_list_randomized() {
    let mut s: BucketList<_> = (0..1000).collect();
    let mut v: Vec<_> = (0..1000).collect();
    let mut seed = 921;
    for _ in 0..10000 {
        let x = (next(&mut seed) % 20) as i32;
        match next(&mut seed) % 6 {
            0 => {
                s.append(x);
                v.push(x);
            }
            1 => {
                let i = next(&mut seed) as usize % (v.len() + 1);
                assert_eq!(s.insert(i as isize, x), Ok(()));
                v.insert(i, x);
            }
            2 if !v.is_empty() => {
                let i = next(&mut seed) as usize % v.len();
                let signed = i as isize - v.len() as isize;
                assert_eq!(s.pop(signed), Some(v.remove(i)));
            }
            3 => {
                s.reverse();
                v.reverse();
            }
            4 => {
                let pos = v.iter().position(|y| *y == x);
                assert_eq!(s.index(&x), pos);
                assert_eq!(s.remove(&x), pos.map(|i| v.remove(i)));
            }
            _ if !v.is_empty() => {
                s[-1] = x;
                *v.last_mut().unwrap() = x;
            }
            _ => {}
        }
        assert_eq!(s.len(), v.len());
        assert_eq!(s.iter().copied().collect::<Vec<_>>(), v);
        assert_eq!(s.count(&x), v.iter().filter(|y| **y == x).count());
        assert_eq!(s.contains(&x), v.contains(&x));
        assert_eq!(s.get(isize::MIN), None);
        assert_eq!(s.insert(v.len() as isize + 1, x), Err(x));
    }
    assert_eq!(s, s.clone());
    s.clear();
    assert_eq!(s.insert(-2, 7), Err(7));
    assert_eq!(s.insert(-1, 7), Ok(()));
    assert_eq!(s.insert(-1, 8), Ok(()));
    assert_eq!(s.pop_last(), Some(7));
    assert_eq!(s.pop_last(), Some(8));
    assert_eq!(s.pop_last(), None);
}
