//! Solution for https://leetcode.com/problems/binary-tree-inorder-traversal
//! 94. Binary Tree Inorder Traversal

// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

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
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];

        Self::traverse(&root, &mut v);

        v
    }

    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(val) = node {
            let a = val.borrow();
            Self::traverse(&a.left, v);
            v.push(a.val);
            Self::traverse(&a.right, v);
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,null,2,3]").into(), vec![1,2,3])]
    #[case(TreeRoot::from("[]").into(), vec![])]
    #[case(TreeRoot::from("[1]").into(), vec![1])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<i32>) {
        let actual = Solution::inorder_traversal(root);
        assert_eq!(actual, expected);
    }
}
