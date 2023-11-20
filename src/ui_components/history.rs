pub struct History<T> {
    curr: T,
    undo: Vec<T>,
    redo: Vec<T>,
    size: usize,
}

impl<T: Clone + Default> History<T> {
    pub fn new(size: usize) -> Self {
        Self {
            curr: T::default(),
            undo: vec![],
            redo: vec![],
            size,
        }
    }

    pub fn get_curr(&mut self) -> &mut T {
        &mut self.curr
    }

    pub fn append(&mut self, elem: T) {
        if !self.redo.is_empty() {
            self.redo.clear();
        }

        if self.undo.len() >= self.size {
            self.undo.rotate_left(1);
            self.undo[self.size - 1] = self.curr.clone();
        } else {
            self.undo.push(self.curr.clone());
        }
        self.curr = elem;
    }

    pub fn reset(&mut self, elem: T) {
        self.curr = elem;
        self.undo.clear();
        self.redo.clear();
    }

    pub fn undo(&mut self) {
        match self.undo.pop() {
            Some(elem) => {
                self.redo.push(self.curr.clone());
                self.curr = elem;
            }
            None => {}
        }
    }

    pub fn redo(&mut self) {
        match self.redo.pop() {
            Some(elem) => {
                self.undo.push(self.curr.clone());
                self.curr = elem;
            }
            None => {}
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}
