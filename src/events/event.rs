pub struct Event<T> {
    kind: T,
    consumed: bool
}

impl<T> Event<T> {
    pub fn new(kind: T) -> Self {
        Event {
            kind,
            consumed: false
        }
    }

    pub fn kind(&self) -> &T {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut T {
        &mut self.kind
    }

    pub fn is_consumed(&self) -> bool {
        self.consumed
    }

    pub fn consume(&mut self) -> bool {
        if self.consumed {
            return false;
        }

        self.consumed = true;
        true
    }
}
