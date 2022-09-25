#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        let mut head = ListNode::new(1);
        head.next = Some(Box::new(ListNode::new(2)));
        head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        head.next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(5)));
        let mut result = Solution::reverse_list(Some(Box::new(head)));
        while let Some(node) = result {
            print!("{} ", node.val);
            result = node.next;
        }
    }
}

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

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut pre, mut cur) = (None, head);
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        pre
    }

    #[allow(dead_code)]
    pub fn reverse_list0(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(
            head: Option<Box<ListNode>>,
            pre: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut node) = head {
                let tail = node.next.take();
                node.next = pre;
                return reverse(tail, Some(node));
            }
            pre
        }
        reverse(head, None)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
