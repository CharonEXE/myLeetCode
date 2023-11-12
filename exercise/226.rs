use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            let mut borrowed_node = node.borrow_mut();

            let left = Solution::invert_tree(borrowed_node.left.take());
            let right = Solution::invert_tree(borrowed_node.right.take());

            borrowed_node.left = right;
            borrowed_node.right = left;
            
            return Rc::clone(&node);
        })

    }
}