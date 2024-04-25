// Link to challenge: https://leetcode.com/problems/two-sum/description/
// Solution based in hint one:
// A really brute force way would be to search for all possible
// pairs of numbers but that would be too slow. Again, it's best to try
// out brute force solutions for just for completeness. It is from these brute
// force solutions that you can come up with optimizations.

mod two_sum;

fn main() {
    let mut nums = vec![2,0,3,1,4];
    let target : i32 = 6;
    let result: Vec<i32>;
    println!("Hello, world!");
    result = two_sum::solution(nums.clone(),target);

    let pos_addend_1 : usize = result[0] as usize;
    let pos_addend_2 : usize = result[1] as usize;

    println!("result:{:?}", result);
    println!("nums[{pos_addend_1}]: {}", nums[pos_addend_1]);
    println!("nums[{pos_addend_2}]: {}", nums[pos_addend_2]);
    println!("target:{target}");
}
