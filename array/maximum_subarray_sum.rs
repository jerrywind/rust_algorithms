// stem from : https://leetcode.cn/problems/maximum-subarray/

pub fn maximum_subarray_sum(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold((i32::MIN, 0), |(ans, pre), n| (ans.max(n.max(n + pre)), n.max(n + pre))).0
}

fn main() {
    println!("Please input your input list, use blank ',' as separator and in [] (e.g ： [1,2,3]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read your input !");
    let input_str = input.trim().trim_matches(|c| c == '[' || c == ']').to_string();
    let i32_list:Vec<i32> = input_str.split(",").map(|s| s.trim().parse().unwrap()).collect();
    let result = maximum_subarray_sum(i32_list);
    println!("maximum sub arrary sum is  : {:?}", result); 
}