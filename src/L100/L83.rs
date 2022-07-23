use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut h = head;
        let mut p1 = h.as_mut().unwrap();
        while let Some(p2) = &mut p1.next {
            if p1.val == p2.val {
                p1.next = p2.next.take();
            } else {
                p1 = p1.next.as_mut().unwrap();
            }
        }
        h
        // let mut prev_val;
        // let mut cur;
        // if let Some(n) = head {
        //     prev_val = n.val;
        //     cur = n;
        // } else {
        //     return head;
        // }
        // let mut new_list = Box::new(ListNode::new(prev_val));

        // while let Some(n) = cur.next {
        //     if n.val != prev_val {
        //         Self::insert(&mut new_list, n.val);
        //         prev_val = n.val
        //     }
        //     cur = n
        // }
        // Some(new_list)
    }

    fn insert(head: &mut Box<ListNode>, val: i32) {
        let new = ListNode::new(val);
        let mut next = &mut head.next;
        while next != &None {
            next = &mut next.as_mut().unwrap().next;
        }
        *next = Some(Box::new(new));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut head = Box::new(ListNode::new(12));
        Solution::insert(&mut head, 78);
        println!("{head:?}")
    }
}
