// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn inorder_traversal(root: OptNode) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = vec![(root, false)];

        while let Some((node, push_now)) = stack.pop() 
        {
            if push_now 
            {
                result.push(node.unwrap().borrow().val);
            } else if let Some(n) = node
            {
                let node_borrowed = n.borrow();
                stack.push((node_borrowed.right.clone(), false));
                stack.push((Some(n.clone()), true));
                stack.push((node_borrowed.left.clone(), false));
            }
        }
        return result;
    }
}