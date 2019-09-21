
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

///
/// 两链表相加
///
/// # Examples
///
/// ```
/// let a2 = ListNode::new(5);
/// let l1 = Some(Box::new(a2));
///
/// let b5 = ListNode::new(5);
/// let l2 = Some(Box::new(b5));
///
/// let c1 = ListNode::new(1);
/// let mut c0 = ListNode::new(0);
/// c0.next = Some(Box::new(c1));
/// let should = Some(Box::new(c0));
/// let res = add_two_numbers(l1, l2);
/// assert_eq!(should, res);
/// ```
///
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut ret = Some(Box::new(ListNode::new(0)));
    let mut tmp = &mut ret;
    let mut up = 0;

    while l1.is_some() || l2.is_some() || up > 0 {
        let v1 = if let Some(l) = &l1 {
            l.val
        } else {
            0
        };

        let v2 = if let Some(l) = &l2 {
            l.val
        } else {
            0
        };

        let sum = v1 + v2 + up;
        up = sum / 10;
        let val = Box::new(ListNode::new(sum % 10));

        tmp.as_mut().unwrap().next = Some(val);
        tmp = &mut tmp.as_mut().unwrap().next;

        l1 = if l1.is_some() {
            l1.unwrap().next
        } else {
            None
        };

        l2 = if l2.is_some() {
            l2.unwrap().next
        } else {
            None
        }
    }

    ret.unwrap().next
}