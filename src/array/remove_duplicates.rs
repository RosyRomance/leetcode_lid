pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0{
            return 0;
        }

        let mut i = 1;
        for n in 1..nums.len(){ 
            if nums[n] > nums[i-1] {
                nums[i] = nums[n];
                i += 1;
            } 
        }
        i as i32
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_remove_duplicates(){
        let mut nums1 = vec![1,1,2];
        let mut nums2 = vec![0,0,1,1,1,2,2,3,3,4];

        assert_eq!(Solution::remove_duplicates(&mut nums1), 2);
        assert_eq!(Solution::remove_duplicates(&mut nums2), 5);
    }
}





