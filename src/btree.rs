use std::cell::RefCell;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn new_direction(val: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hello")?;
        let curr = self.left.as_mut().unwrap().borrow_mut().deref_mut();
        while curr.left.is_some() {}
    }
}

fn insert(tree: &mut TreeNode, val: i32) {
    if tree.left.is_none() {
        tree.left = TreeNode::new_direction(val);
    } else if tree.right.is_none() {
        tree.right = TreeNode::new_direction(val);
    } else {
        insert(&mut tree.left.as_mut().unwrap().borrow_mut(), val);
    }
}

fn bfs(tree: TreeNode, val: i32) -> bool {
    todo!()
}
