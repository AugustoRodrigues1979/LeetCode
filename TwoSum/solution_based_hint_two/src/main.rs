// Link to challenge: https://leetcode.com/problems/two-sum/description/
// Solution based in hint two:
// So, if we fix one of the numbers, say x, we have to
// scan the entire array to find the next number y which is value - x
// where value is the input parameter. Can we change our array somehow so
// that this search becomes faster?
mod two_sum;

fn main() {
    let nums : Vec<i32> = vec![3,2,4];
    let target: i32 = 6;
    let result: Vec<i32>;

    result = two_sum::solution(nums.clone(), target);

    let pos_addend_1: usize = result[0] as usize;
    let pos_addend_2: usize = result[1] as usize;

    println!("nums:{:?}", nums);
    println!("result:{:?}", result);
    println!("nums[{pos_addend_1}]: {}", nums[pos_addend_1]);
    println!("nums[{pos_addend_2}]: {}", nums[pos_addend_2]);
    println!("target:{target}");
}
