use std::cell;
/**
 * cell uses
 * + UnsafeCell
 * + Cell
 * + RefCell
 * + Mutex
*/
fn main() {
    /*
        Cell with get method gives you the value.
        NOT THE POINTER
        So no one else can have it.
        Cell should not be used with threads.
        Every check is at compile time.
        You should use it with Copy values and vals that are easy to clone.
    */
    let x = cell::Cell::new(5);
    let val_of_x = x.get();
}

use std::cell::UnsafeCell;
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(val: T) -> Self {
        Cell {
            value: UnsafeCell::new(val),
        }
    }

    pub fn set(&self, value: T) {
        // we know no-one else is concurently mutating self
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use std::slice::SliceIndex;

    use super::Cell;
    #[test]
    fn my_cell() {
        let c = Cell::new(5);
        c.set(123);
        let x = c.get();
    }
}
