fn main() {}

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    val: T,
    next: Option<Box<LinkedList<T>>>,
}

use std::ops::Add;

impl<T> LinkedList<T> {
    pub fn new(val: T) -> Self {
        LinkedList::<T> { val, next: None }
    }
    pub fn new_next(val: T) -> Option<Box<LinkedList<T>>> {
        Some(Box::new(LinkedList::<T> { val, next: None }))
    }
}

impl<T> LinkedList<T>
where
    T: Add + Default + Add<Output = T>,
{
    pub fn sum(self) -> T {
        let mut curr = Box::new(self);
        let mut sum: T = T::default();
        while curr.next.is_some() {
            sum = sum + curr.val;
            curr = curr.next.unwrap();
        }
        sum + curr.val
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn new_int() {
        let x = LinkedList::new(5);
        assert_eq!(
            x,
            LinkedList {
                val: 5i32,
                next: None
            }
        );
    }

    #[test]
    fn new_str() {
        assert_eq!(
            LinkedList::new("123"),
            LinkedList {
                val: "123",
                next: None
            }
        );
    }

    #[test]
    fn new_next_int() {
        assert_eq!(LinkedList::new_next(5), Some(Box::new(LinkedList::new(5))))
    }

    #[test]
    fn sum() {
        let mut l = LinkedList::new(1);
        l.next = LinkedList::new_next(2);
        l.next.as_mut().unwrap().next = LinkedList::new_next(3);
        assert_eq!(l.sum(), 6);
    }
}
