/// https://leetcode.com/problems/two-sum/editorial/
///
/// # Problem
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///

use std::collections::VecDeque;
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut queue = VecDeque::new();
        for val in &nums {
            queue.push_back(val);
        }

        let index1 = 0;
        loop {
            let n = queue.pop_front().unwrap();
            let n_needed = target - n;
            if queue.contains(&n_needed) {
                let index2 = queue.iter().position(|&x| &x == n_needed);
                return vec![index1 as i32, (index1+index2+1) as i32];
            }
        }
        vec![0, 1]

    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_two_sum(){
        let input1 = vec![2, 7, 11, 5];
        let input2 = vec![3, 2, 4];
        let input3 = vec![3, 3];

        assert_eq!(Solution::two_sum(input1, 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(input2, 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(input3, 6), vec![0, 1]);
    }
}