//! Solution for https://leetcode.com/problems/merge-bsts-to-create-single-bst
//! 1932. Merge BSTs to Create Single BST

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
use cargo_leet::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Fill in body")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(todo!("[[2,1],[3,2,5],[5,4]]"), todo!("Expected Result"))]
    #[case(todo!("[[5,3,8],[3,2,6]]"), todo!("Expected Result"))]
    #[case(todo!("[[5,4],[3]]"), todo!("Expected Result"))]
    fn case(
        #[case] trees: Vec<Option<Rc<RefCell<TreeNode>>>>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        let actual = Solution::can_merge(trees);
        assert_eq!(actual, expected);
    }
}
