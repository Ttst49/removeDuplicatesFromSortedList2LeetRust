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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.map(|mut ln|{
        let mut current = ln.as_mut();
        while let Some(next) = current.next.as_mut(){
            if current.val == next.val {
                current.next = next.next.take()
            }else {
                current = current.next.as_mut().unwrap();
            }
        }
        ln
    })
}

fn main() {
    let head:Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
    delete_duplicates(head);
}
