#[derive(Default)]
pub struct Stack<T> {
    base: Vec<T>,
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            base: iter.into_iter().collect::<Vec<_>>(),
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            base: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn push(&mut self, elt: T) {
        self.base.push(elt)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.base.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.base.last()
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }
}

impl<T> Stack<T> {
    #[allow(dead_code)]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.base.iter(),
        }
    }
}

pub struct Iter<'a, T: 'a> {
    next: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.next_back()
    }
}

pub struct IntoIter<T> {
    it: Stack<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { it: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_test() {
        let stk = Stack {
            base: vec![1, 2, 3, 4, 5],
        };
        let mut tmp = 0;
        for &i in stk.iter() {
            tmp += i;
        }
        assert_eq!(tmp, 15);
    }

    #[test]
    fn from_slice_test() {
        let v = vec![1, 2, 3, 4];
        let stk = v.into_iter().collect::<Stack<_>>();
        assert_eq!(stk.len(), 4);
    }
}
