struct Solution;
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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = head.take().map(|node| {
            head = node.next;
            Box::new(ListNode {
                val: node.val,
                next: None,
            })
        });
        while let Some(node) = head.take() {
            head = node.next;
            let new_node = Box::new(ListNode {
                val: node.val,
                next: ans.take(),
            });
            ans = Some(new_node)
        }

        ans
    }
}

fn main() {
    let mut head = Some(Box::new(ListNode::new(4)));
    println!("{:?}", head);
    let arr = vec![3, 2, 4, 1];
    for val in arr {
        let new_node = Box::new(ListNode {
            val: val,
            next: head.take(),
        });
        head = Some(new_node);
    }
    println!("{:?}", head);
    let result = Solution::reverse_list(head);
    println!("{:?}", result);
}
