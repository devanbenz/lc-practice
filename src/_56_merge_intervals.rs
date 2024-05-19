//! Solution for https://leetcode.com/problems/merge-intervals
//! 56. Merge Intervals

use std::collections::VecDeque;
use std::thread::current;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret_vec = Vec::new();
        let mut cur_vec = Vec::new();

        let mut ints = intervals.clone();
        ints.sort_by(|a, b| a[0].cmp(&b[0]));

        for interval in ints {
            if !cur_vec.is_empty() {
                if cur_vec[1] < interval[0] {
                    ret_vec.push(cur_vec.clone());
                    cur_vec.clear();
                    cur_vec.push(interval[0]);
                    cur_vec.push(interval[1]);
                } else if interval[1] > cur_vec[1] {
                    cur_vec[1] = interval[1];
                }
            } else {
                cur_vec.push(interval[0]);
                cur_vec.push(interval[1]);
            }
        }

        if !cur_vec.is_empty() {
            ret_vec.push(cur_vec);
        }

        ret_vec
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec ! [vec ! [1, 3], vec ! [2, 6], vec ! [8, 10], vec ! [15, 18]], vec ! [vec ! [1, 6], vec ! [8, 10], vec ! [15, 18]])]
    #[case(vec ! [vec ! [1, 4], vec ! [4, 5]], vec ! [vec ! [1, 5]])]
    #[case(vec ! [vec ! [2, 3], vec ! [5, 6], vec![1,10]], vec ! [vec ! [1, 10]])]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::merge(intervals);
        assert_eq!(actual, expected);
    }
}
