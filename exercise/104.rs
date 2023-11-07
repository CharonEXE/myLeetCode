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
use std::cmp::Ordering;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        let mut depth: i32 = 0;

        if root == None {
            return depth;
        }

        match Rc::try_unwrap(root.unwrap())
        {
            Ok(thing) => {
                depth += 1;
                let mut depth_right: i32 = 0;
                let mut depth_left: i32 = 0;
                let current = thing.into_inner();
                
                if current.right != None {
                    depth_right = Solution::max_depth(current.right);
                }
                if current.left != None {
                    depth_left = Solution::max_depth(current.left);
                }

                match depth_right.cmp(&depth_left){
                    Ordering::Less => return (depth + depth_left),
                    _ => return (depth + depth_right),
                }
            },
            _ => return depth,
        };

        return depth;
    }
}