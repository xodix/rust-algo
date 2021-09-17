use std::fmt::{self, Display};
/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut curr = self;
        let mut res = curr.val.to_string();
        while curr.next.is_some() {
            res += " -> ";
            res += &curr.val.to_string();
            curr = curr.next.as_ref().unwrap();
        }
        write!(f, "{}", res)
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(arr: Vec<i32>) -> Self {
        let mut node = Box::new(ListNode::new(arr[0]));
        let mut curr = &mut node;
        for i in 1..arr.len() {
            curr.next = ListNode::new_next(arr[i]);
            curr = curr.next.as_mut().unwrap();
        }
        *node
    }
}

impl Into<Vec<i32>> for ListNode {
    fn into(self) -> Vec<i32> {
        let mut curr = self;
        let mut res = Vec::new();
        while curr.next.is_some() {
            res.push(curr.val);
            curr = *curr.next.unwrap();
        }
        res
    }
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn new_next(val: i32) -> Option<Box<Self>> {
        Some(Box::new(ListNode::new(val)))
    }
}
