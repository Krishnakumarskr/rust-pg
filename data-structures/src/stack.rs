pub struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.stack.pop();
    }

    pub fn new() -> Self  {
        return Stack {
            stack: Vec::new()
        }
    }

    pub fn length(&self) -> usize {
        return self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}