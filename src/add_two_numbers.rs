#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_rec(l1, l2, 0)
}


pub fn add_two_numbers_rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    let num1 = match &l1 {
        None => 0,
        Some(x) => x.val
    };

    let num2 = match &l2 {
        None => 0,
        Some(x) => x.val
    };

    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let num = num1 + num2 + carry;
    let val = num / 10;
    let rem = num % 10;

    let next1 = l1.map_or(None, |node| node.next);
    let next2 = l2.map_or(None, |node| node.next);

    return Some(Box::new(ListNode { val: rem, next: add_two_numbers_rec(next1, next2, val) }));
}

pub fn to_int(l: Option<Box<ListNode>>) -> i32 {
    let mut head = l;
    let mut num1 = 0;
    let mut i = 0;
    while head.is_some() {
        let unwrapped = head.unwrap();
        num1 += unwrapped.val * 10_i32.pow(i);
        head = unwrapped.next;
        i += 1;
    }
    num1
}
