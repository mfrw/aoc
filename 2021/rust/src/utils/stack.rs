#[derive(Default)]
pub struct Stack<T> {
    base: Vec<T>,
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
