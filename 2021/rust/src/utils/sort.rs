use std::cmp::Ordering;

extern crate alloc;

pub trait Sorter<T>: Iterator<Item = T> + Sized
where
    T: Ord,
{
    fn sort(self) -> alloc::vec::IntoIter<T> {
        self.sort_by(|a, b| a.cmp(&b))
    }

    fn sort_by_key<F, K>(self, mut f: F) -> alloc::vec::IntoIter<T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.sort_by(|a, b| f(a).cmp(&f(b)))
    }

    fn sort_by_cached_key<F, K>(self, f: F) -> alloc::vec::IntoIter<T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut v = Vec::from_iter(self.into_iter());
        v.as_mut_slice().sort_by_cached_key(f);
        v.into_iter()
    }

    fn sort_by<F>(self, compare: F) -> alloc::vec::IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut v = Vec::from_iter(self.into_iter());
        v.as_mut_slice().sort_by(compare);
        v.into_iter()
    }

    fn sort_unstable_by<F>(self, compare: F) -> alloc::vec::IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut v = Vec::from_iter(self.into_iter());
        v.as_mut_slice().sort_unstable_by(compare);
        v.into_iter()
    }

    fn sort_unstable_by_key<F, K>(self, mut f: F) -> alloc::vec::IntoIter<T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.sort_unstable_by(|a, b| f(a).cmp(&f(b)))
    }

    fn sort_unstable(self) -> alloc::vec::IntoIter<<Self as IntoIterator>::Item> {
        self.sort_unstable_by(|a, b| a.cmp(&b))
    }
}

impl<I, T> Sorter<T> for I
where
    I: Iterator<Item = T>,
    T: Ord,
{
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn sort_test() {
        let v = [5, 4, 3, 2, 1];
        let mut vit = v.iter().rev();
        let mut it = v.iter().sort();
        for _ in 0..6 {
            assert_eq!(it.next(), vit.next());
        }
    }

    #[test]
    fn sort_by_test() {
        let mut v = [2, 15, 4, 3];
        let vtest = [2, 15, 4, 3];
        v.sort();
        let mut vit = v.iter();
        let mut it = vtest.iter().sort_by(|a, b| a.cmp(&b));
        for _ in 0..6 {
            assert_eq!(it.next(), vit.next());
        }
    }
}
