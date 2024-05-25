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

pub fn delete_duplicates(head: Option<Box<ListNode>>)-> Option<Box<ListNode>>{
    fn go(node: Option<Box<ListNode>>, prev: Option<i32>) -> Option<Box<ListNode>> {
        if let Some(mut n) = node {
            let x = Some(n.val);
            if x == prev {
                go(n.next, prev)
            } else {
                match n.next {
                    Some(n2) if n2.val == n.val => go(Some(n2), x),
                    _ => {
                        n.next = go(n.next, x);
                        Some(Box::new(*n))
                    }
                }
            }
        } else {
            None
        }
    }

    go(head, None)
}

fn main() {
    let head:Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
    delete_duplicates(head);
}
