use std::cmp;
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() {
            self.min_stack.push(val);
        } else {
            self.min_stack
                .push(cmp::min(*self.min_stack.last().unwrap(), val))
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}
fn main() {
    let val = 3;
    let mut obj = MinStack::new();
    obj.push(val);
    obj.pop();
    let ret_3: i32 = obj.top();
    let ret_4: i32 = obj.get_min();
}
