struct Solution;
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
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, mut ans: Vec<i32>) -> Vec<i32> {
        match node {
            Some(n) => {
                ans.push(n.borrow().val);
                ans = Self::preorder(&n.borrow().left, ans);
                Self::preorder(&n.borrow().right, ans)
            }
            None => ans,
        }
    }

    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let ans = Vec::new();
        Self::preorder(&root, ans)
    }
}
fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    let tree = tree.unwrap();
    tree.borrow_mut().left = left;
    tree.borrow_mut().right = right;
    let tree = Some(tree);
    println!("{:?}", tree);
    let result = Solution::preorder_traversal(tree);
    println!("{:?}", result);
}
