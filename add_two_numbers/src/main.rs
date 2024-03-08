fn main() {
    let solution = Solution::add_two_numbers(
        Solution::node_from_vector(vec![8, 2, 3, 4, 5]),
        Solution::node_from_vector(vec![9, 4, 3, 2, 1]),
    );
    println!("{:?}", solution)
}

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let v1 = Self::node_to_vector(l1);
        let v2 = Self::node_to_vector(l2);

        let v_result = Solution::join_vectors(v1, v2);

        Self::node_from_vector(v_result)
    }

    fn join_vectors(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
        let mut joined: Vec<i32> = vec![]; //TODO could this vector be known in size?
        let mut i1 = v1.iter();
        let mut i2 = v2.iter();
        let mut overflow = false;

        while let Some(n1) = i1.next() {
            let mut result = *n1;

            let n2 = i2.next();
            result = n2.unwrap_or(&0) + result;

            if overflow {
                result = result + 1;
                overflow = false;
            }

            if result >= 10 {
                overflow = true;
                result = result - 10;
            }

            joined.push(result)
        }

        while let Some(n) = i2.next() {
            let mut result = *n;

            if overflow {
                result = result + 1;
                overflow = false;
            }

            if result >= 10 {
                overflow = true;
                result = result - 10;
            }

            joined.push(result)
        }
        if overflow {
            joined.push(1)
        }

        joined
    }

    fn node_to_vector(node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec: Vec<i32> = vec![];
        let mut current_node = node;

        while let Some(node) = current_node {
            vec.push(node.val);
            current_node = node.next;
        }
        vec
    }

    fn node_from_vector(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut i = vec.iter().rev();
        let mut node = ListNode::new(*i.next().expect("Error: empty iterator"));

        while let Some(number) = i.next() {
            let mut prv_node = ListNode::new(*number);
            prv_node.next = Some(Box::new(node));
            node = prv_node;
        }

        Some(Box::new(node))
    }
}
