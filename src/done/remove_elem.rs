mod list_node;
use list_node::ListNode;
fn main() {
    let x = &mut Box::new(ListNode::from(vec![
        2, 1, 2, 3, 4, 5, 6, 2, 3, 3, 2, 2, 6, 2,
    ]));
    println!("{}", remove_elements(x, 2));
}

fn remove_elements(head: &mut Box<ListNode>, useless_num: i32) -> ListNode {
    let mut curr = head;
    // After a few hours I elected to leve space complexity at O(n) becouse i have to return LinkedList
    let mut res_node = Box::new(ListNode::new(0));
    let mut res_curr = &mut res_node;
    while curr.next.is_some() {
        if curr.val != useless_num {
            res_curr.val = curr.val;
            res_curr.next = ListNode::new_next(0);
            res_curr = res_curr.next.as_mut().unwrap();
        }
        curr = curr.next.as_mut().unwrap();
    }
    *res_node
}
