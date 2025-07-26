struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.next == None || list2.next == None {
            
        }
    }
}




#[test]
fn test_merge_two_lists() {
    let list1 = vec![1,2,4];
    let list2 = vec![1,3,4];
    assert_eq!(Solution::merge_two_lists(list1, list2), vec![1,1,2,3,4,4]);

    let list1 = vec![];
    let list2 = vec![];
    assert_eq!(Solution::merge_two_lists(list1, list2), vec![]);

    let list1 = vec![];
    let list2 = vec![0];
    assert_eq!(Solution::merge_two_lists(list1, list2), vec![0]);
}