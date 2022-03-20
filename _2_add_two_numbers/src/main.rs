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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut c1 = l1;
    let mut c2 = l2;

    let mut s = 0;
    let mut sum: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut node = sum.as_mut();
    while c1.is_some() || c2.is_some() || s != 0 {
        if let Some(c) = c1 {
            s += c.val;
            c1 = c.next;
        }

        if let Some(c) = c2 {
            s += c.val;
            c2 = c.next;
        }

        let d = s % 10;
        s /= 10;

        let n = node.unwrap();
        n.next = Some(Box::new(ListNode::new(d)));
        node = n.next.as_mut();
    }

    sum.unwrap().next
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_the_sum_as_a_linked_list_in_reverse_order() {
        //Input: l1 = [2,4,3], l2 = [5,6,4]
        //Output: [7,0,8]
        let mut l1 = Box::new(ListNode::new(2));
        l1.next = Some(Box::new(ListNode::new(4)));
        l1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        let mut l2 = Box::new(ListNode::new(5));
        l2.next = Some(Box::new(ListNode::new(6)));
        l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

        let result = add_two_numbers(Some(l1), Some(l2));

        assert_eq!(result.clone().unwrap().val, 7);
        assert_eq!(result.clone().unwrap().next.unwrap().val, 0);
        assert_eq!(result.unwrap().next.unwrap().next.unwrap().val, 8);
    }

    #[test]
    fn should_return_the_sum_as_a_linked_list_in_reverse_order2() {
        //[9] [1,9,9]
        //Output: [0,0,0,1]
        let l1 = Box::new(ListNode::new(9));

        let mut l2 = Box::new(ListNode::new(1));
        l2.next = Some(Box::new(ListNode::new(9)));
        l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));

        let result = add_two_numbers(Some(l1), Some(l2));

        assert_eq!(result.clone().unwrap().val, 0);
        assert_eq!(result.clone().unwrap().next.unwrap().val, 0);
        assert_eq!(result.clone().unwrap().next.unwrap().next.unwrap().val, 0);
        assert_eq!(
            result
                .unwrap()
                .next
                .unwrap()
                .next
                .unwrap()
                .next
                .unwrap()
                .val,
            1
        );
    }
}

/* i32的最大值为2147483647，要注意输入条件*/
