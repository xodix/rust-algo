// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// use bfs + insert the rest
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr_node1 = l1.unwrap();
    let mut curr_node2 = l2.unwrap();
    let mut decimal_rest = 0;
    let mut result_vec: Vec<i32> = vec![];

    // make linked lists same length
    while !(curr_node1.next.is_none() && curr_node2.next.is_none()) {
        if curr_node1.next.is_none() {
            curr_node1.next = Some(Box::new(ListNode::new(0)));
        } else {
            curr_node2.next = Some(Box::new(ListNode::new(0)));
        }
        curr_node1.next = Some(Box::new(ListNode::new(0)));
    }

    // add results of linked list addition to result vector
    while !curr_node1.next.is_none() {
        let sum = curr_node1.val + curr_node2.val;
        if sum + decimal_rest > 10 {
            result_vec.push(sum - 10 + decimal_rest);
            decimal_rest = 1;
        } else {
            result_vec.push(sum + decimal_rest);
            decimal_rest = 0;
        }
        curr_node1 = curr_node1.next.unwrap();
        curr_node2 = curr_node2.next.unwrap();
    }

    Some(Box::new(result_node))
}
