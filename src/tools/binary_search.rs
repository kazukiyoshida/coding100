use std::fmt::Debug;

pub trait BinarySearch<T> {
    fn contains(&self, k: &T) -> bool;

    //
    // return index              *
    // range                     |----------
    // bool:           x x x x x o o o o o o
    //
    // return index     *   index = 0
    // range            |----------
    // bool:            o o o o o o
    //
    // return index                 *   index = None
    // range                        |----------
    // bool:            x x x x x x
    //
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> Option<usize>;

    //
    // return index              *
    // range           ----------|
    // bool:           o o o o o o x x x x x
    //
    // return index               *   index = vec.len
    // range            ----------|
    // bool:            o o o o o o
    //
    // return index       *   index = None
    // range           ---|
    // bool:                x x x x x x
    //
    fn upper_bound(&self, f: impl Fn(&T) -> bool) -> Option<usize>;
}

impl<T: PartialEq + Debug + PartialOrd> BinarySearch<T> for [T] {
    fn contains(&self, k: &T) -> bool {
        let mut left: isize = -1;
        let mut right = self.len() as isize;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if &self[mid as usize] == k {
                return true;
            } else if &self[mid as usize] > k {
                right = mid;
            } else {
                left = mid;
            }
        }
        false
    }

    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> Option<usize> {
        let mut left: isize = -1;
        let mut right = self.len() as isize;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if f(&self[mid as usize]) {
                right = mid;
            } else {
                left = mid;
            }
        }
        if self.len() - 1 < right as usize {
            None
        } else {
            Some(right as usize)
        }
    }

    fn upper_bound(&self, f: impl Fn(&T) -> bool) -> Option<usize> {
        let mut left: isize = -1;
        let mut right = self.len() as isize;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if f(&self[mid as usize]) {
                left = mid;
            } else {
                right = mid;
            }
        }
        if !f(&self[0]) {
            None
        } else {
            Some(left as usize)
        }
    }
}

#[test]
fn binary_search_test() {
    // contains
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&5), true);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&1), true);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&2), true);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&9), true);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&10), false);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].contains(&99), false);

    // lower_bound
    assert_eq!(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0].lower_bound(|&k| k > 5),
        Some(5)
    );
    assert_eq!(
        vec![2, 6, 4, 12, 8, 24].lower_bound(|&k| k % (2 * 2 * 2) == 0),
        Some(4)
    );
    assert_eq!(vec![1, 2, 3, 4, 5].lower_bound(|&k| k > 0), Some(0));
    assert_eq!(vec![1, 2, 3, 4, 5].lower_bound(|&k| k > 5), None);

    // upper_bound
    assert_eq!(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0].upper_bound(|&k| k <= 5),
        Some(4)
    );
    assert_eq!(vec![1, 2, 3, 4, 5].upper_bound(|&k| k > 0), Some(4));
    assert_eq!(vec![1, 2, 3, 4, 5].upper_bound(|&k| k < 5), Some(3));
}
