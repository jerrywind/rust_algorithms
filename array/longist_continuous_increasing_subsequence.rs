// stem from : https://leetcode.cn/problems/longest-continuous-increasing-subsequence
// not really the same, the result here is to return the longist continuous increasing sub sequence.

pub fn find_length_of_lcis(nums: Vec<i32>) -> Vec<i32> {
    let mut max_l = 0;
    let mut max_start = 0;
    let mut cur_l = 0;
    let mut i = 1;
    while i <= nums.len(){
        if i == nums.len() || &nums[i] <= &nums[i-1]{
            let cur_len = i - cur_l;
            if cur_len > max_l {
                max_l = cur_len;
                max_start = cur_l;
            }
            cur_l = i;
        }
        i += 1;
    }
    let slice =  Vec::from(&nums[max_start..max_start+max_l]);
    slice
}

fn main() {
    println!("Please input your input list, use blank ' ' as separator(e.g '1 2 3')：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read your input !");
    let input_str = input.trim().to_string();
    let i32_list:Vec<i32> = input_str.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let result = find_length_of_lcis(i32_list);
    println!("Longist continuous increasing subsequence is : {:?}", result);
}