pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0,1];

    for n in 0..nums.len() {
        let addend_one = nums[n].clone();
        for k in n+1..nums.len() {
            let addend_two = nums[k].clone();
            if addend_one + addend_two == target {
                result[0] = n as i32;
                result[1] = k as i32;
                return result;
            }
        }
    }
    result
}