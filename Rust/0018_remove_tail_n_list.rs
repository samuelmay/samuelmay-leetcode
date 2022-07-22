//Definition for singly-linked list.
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

fn list_from_vec(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut list: Option<Box<ListNode>> = None;
    for i in vector {
        let node = ListNode { next: list, val: i };
        list = Some(Box::new(node));
    }
    return list;
}

fn print_list(mut list: &Option<Box<ListNode>>) {
    print!("(");
    while let Some(ref tail) = list {
        print!("{}",tail.val);
        match tail.next {
            Some(_) => { print!(",") },
            None => {}
        }
        list = &tail.next;
    }
    println!(")");
}

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (_, list) = Self::remove_nth_from_end_count(head,n);
        return list;
    }
    
    fn remove_nth_from_end_count(list: Option<Box<ListNode>>, n: i32) -> (i32, Option<Box<ListNode>>) {
        match list {
            Some(mut node) => {
                let (tail_count, tail) = Self::remove_nth_from_end_count(node.next.take(), n);
                let count = tail_count + 1;
                if n == count {
                    (count,tail)
                } else {
                    node.next = tail;
                    (count,Some(node))
                }
            },
            None => {
                (0,None)
            }
        }
    }


}

fn main() {
    let test = list_from_vec(vec![1,2,3,4]);
    print_list(&test);
    let res = Solution::remove_nth_from_end(test,4);
    print_list(&res);
}
