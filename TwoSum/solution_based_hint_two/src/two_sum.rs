// An iterative shell sort function.
// Source: https://www.treinaweb.com.br/blog/conheca-os-principais-algoritmos-de-ordenacao
fn shell_sort(nums: Vec<i32>, size: i32) -> Vec<i32> {
    let mut j: i32;
    let mut value: i32;
    let mut gap:i32 = 1;
    let mut new_array: Vec<i32> = nums.clone();

    while gap < size {
        gap = 3 * gap + 1;
    }

    while gap > 1 {
        gap /= 3;
        for i in gap..size {
            value = new_array[i as usize];
            j = i - gap;
            while (j as i32 >= 0 as i32) && value < new_array[j as usize] {
                new_array[(j + gap) as usize] = new_array[j as usize];
                j -= gap;
            }
            new_array[(j + gap) as usize] = value
        }
    }

    new_array
}

// An iterative binary search function.
// Source: https://www.geeksforgeeks.org/binary-search/
fn binary_search(
    arr: Vec<i32>,
    mut left_side: i32,
    mut rigth_side: i32,
    target_search: i32,
) -> i32 {
    while left_side <= rigth_side {
        let middle_side = left_side + (rigth_side - left_side) / 2;
        // Check if x is present at mid
        if arr[middle_side as usize] == target_search {
            return middle_side as i32;
        }

        // If x greater, ignore left half
        if arr[middle_side as usize] < target_search {
            left_side = middle_side + 1;
        } else {
            rigth_side = middle_side - 1;
        }
    }

    // If we reach here, then element was not present
    return -1;
}

pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 1];
    let length: i32 = nums.len() as i32;
    let sorted_nums = shell_sort(nums.clone(), length);

    let mut addend_value_1 : i32 = nums[0];
    let mut addend_value_2 : i32 = nums[1];
    let mut search_result: i32;
    for i in 0..length {
        let rest_of_sum = target - nums[i as usize];

        search_result = binary_search(
            sorted_nums.clone(),
            0,
            (sorted_nums.len() - 1) as i32,
            rest_of_sum
        );
        if search_result != -1 {
            addend_value_1 = target - rest_of_sum;
            addend_value_2 = rest_of_sum;
            break;
        }
    }

    for i in 0..length {
        if nums[i as usize] == addend_value_1 {
            result[0] = i;
        }
        if nums[i as usize] == addend_value_2 {
            result[1] = i;
        }
    }

    result
}
