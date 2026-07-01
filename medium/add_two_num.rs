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

fn main() {
    let l1 = Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    });

    let l2 = Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    });

    let mut res = add_two_numbers(Some(l1), Some(l2));
    while let Some(node) = res {
        println!("{}", node.val);
        res = node.next
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry: i32 = 0;
    let mut curr_node_l1 = &l1;
    let mut curr_node_l2 = &l2;
    let mut head = Box::new(ListNode::new(0));
    let mut curr_res_node = &mut head;
    loop {
        let v1 = curr_node_l1.as_ref().map_or(0, |node| node.val);
        let v2 = curr_node_l2.as_ref().map_or(0, |node| node.val);
        curr_res_node.val = v1 + v2 + carry;
        carry = 0;
        if curr_res_node.val >= 10 {
            curr_res_node.val = curr_res_node.val % 10;
            carry = 1;
        }
        let l1_has_next = curr_node_l1
            .as_ref()
            .map_or(false, |node| node.next.is_some());
        let l2_has_next = curr_node_l2
            .as_ref()
            .map_or(false, |node| node.next.is_some());
        if l1_has_next  || l2_has_next || carry > 0
        {
            curr_node_l1 = curr_node_l1.as_ref().map_or(&None, |node| &node.next);
            curr_node_l2 = curr_node_l2.as_ref().map_or(&None, |node| &node.next);

            curr_res_node.next = Some(Box::new(ListNode::new(0)));
            curr_res_node = curr_res_node.next.as_mut().unwrap();
        } else {
            break;
        }
    }
    Some(head)
}
