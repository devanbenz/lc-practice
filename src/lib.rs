pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
pub mod _1932_merge_bsts_to_create_single_bst;
pub mod _1_two_sum;
pub mod _459_repeated_substring_pattern;
pub mod _56_merge_intervals;
pub mod binary_tree_inorder_traversal;
