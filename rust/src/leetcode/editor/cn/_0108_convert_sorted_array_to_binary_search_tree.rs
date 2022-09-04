#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("108、convert_sorted_array_to_binary_search_tree");
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
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
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    #[allow(dead_code)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn sorted_slice_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.len() == 0 {
                return None;
            }
            let mid = nums.len() / 2;
            let mut root = TreeNode::new(nums[mid]);
            root.left = sorted_slice_to_bst(&nums[..mid]);
            root.right = sorted_slice_to_bst(&nums[mid + 1..]);
            Some(Rc::new(RefCell::new(root)))
        }
        sorted_slice_to_bst(&nums)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
