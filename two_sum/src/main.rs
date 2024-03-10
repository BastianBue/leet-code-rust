fn main() {
    println!("Hello, world!");

    println!("{:?}",Solution::two_sum(vec![2, 4, 6, 7,3, 324,23, 1, 3], 5))
}

struct Solution{
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pointer = 0;
        loop{
            match Self::match_after_index(&nums, target, pointer) {
                Some(matching_index) =>  return vec![pointer.try_into().unwrap(), matching_index.try_into().unwrap()],
                None => {
                    pointer = pointer +1;
                }
            };
        }
    }

    fn match_after_index(nums: &Vec<i32>, target: i32, index: usize) -> Option<usize>{
        let starting_number = nums[index];
     for i in (index + 1)..nums.len(){
         if nums[i] + starting_number == target {
             return Some(i)
         }
     }   
     None
    }
}