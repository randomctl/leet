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
fn main() {}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut curr = &head;
    let mut i: usize = 0usize;
    while let Some(node) = curr {
        i += 1usize;
        curr = &node.next;
    }

    if n as usize == i {
        return head.unwrap().next;
    } else {
        let index = i - n as usize;
        let mut link = &mut head;

        for _ in 0..index {
            link = &mut link.as_mut().unwrap().next;
        }

        let mut removed = link.take().unwrap();
        *link = removed.next.take();
        return head;
    }
}
