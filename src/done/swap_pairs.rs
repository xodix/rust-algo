fn main() {
    let vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut l = Some(Box::new(ListNode::from(vals)));
    swap_pairs(&mut l);
    println!("{}", l.unwrap());
}

fn swap_pairs(head: &mut Option<Box<ListNode>>) {
    let mut curr = head.as_mut().unwrap();
    while curr.next.is_some() {
        std::mem::swap(&mut curr.val, &mut curr.next.as_mut().unwrap().val);
        curr = curr.next.as_mut().unwrap();
        if curr.next.is_none() {
            return;
        }
        curr = curr.next.as_mut().unwrap();
    }
}
