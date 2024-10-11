
// stem from : https://algo.course.rs/string/manacher.html

/**
 * manacher算法的时间复杂度分析 (s的长度为n):
 *  1. 增广输入字符串为extend_chars : 2n + 1
 *  2. 第一个for iterator : 2n + 1
 *  3. while 循环的中累加总次数为2n+1 
 */     


/**** Simple Test 
 * jint天气不错不气天123321
 * -> 天气不错不气天
 * 
 * jint天气不错错不气天1235521289
 * -> 天气不错错不气天
 * 
 * jint天气不错不气天12355321123553212
 * -> 1235532112355321
 * 
 * 11
 * -> 11
 * 
 ****/

 pub fn manacher(s: String) -> String {
    let l = s.len();
    if l < 2 {
        return s;
    }

    // MEMO: We need to detect odd palindrome as well,
    // therefore, inserting dummy string so that
    // we can find a pair with dummy center character.
    //let mut extend_chars: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);
    let mut extend_chars:Vec<char> = Vec::new();
    for c in s.chars() {
        extend_chars.push('#');
        extend_chars.push(c);
    }
    extend_chars.push('#');

    // List: storing the length of palindrome at each index of string
    let mut length_of_palindrome = vec![1usize; extend_chars.len()];
    // Integer: Current checking palindrome's center index
    let mut current_center: usize = 0;
    // Integer: Right edge index existing the radius away from current center
    let mut right_from_current_center: usize = 0;
    let mut max_center : usize = 0;
    let mut max_length : usize = 0;
    for i in 0..extend_chars.len() {
        // 1: Check if we are looking at right side of palindrome.
        if right_from_current_center > i {
            // 1-1: If so copy from the left side of palindrome.
            // If the value + index exceeds the right edge index, we should cut and check palindrome later #3.
            length_of_palindrome[i] = std::cmp::min(
                right_from_current_center - i,
                length_of_palindrome[2 * current_center - i],
            );
        }

        // Integer: Current radius from checking index
        // If it's copied from left side and more than 1,
        // it means it's ensured so you don't need to check inside radius.
        // 2: Checking palindrome.
        // Need to care about overflow usize.
        while i >= length_of_palindrome[i] &&  i+length_of_palindrome[i] < extend_chars.len() && extend_chars[i - length_of_palindrome[i]] == extend_chars[i + length_of_palindrome[i]]{
            length_of_palindrome[i] += 1;
        }

        if  i + length_of_palindrome[i] >= right_from_current_center {
            right_from_current_center = i + length_of_palindrome[i];
            current_center = i;
            if length_of_palindrome[i] >= max_length {
                max_length = length_of_palindrome[i];
                max_center = current_center;
            }
        }
    }
    //println!("{:?}", length_of_palindrome);
    // to avoid negative numbers, for usize , it is better to add then minus.
    let left_idx = max_center + 1 - max_length;
    let right_idx = max_center + max_length ;
    //println!("string : {}, max_center: {}, max_length: {}, left : {}, right : {}", extend_chars.iter().collect::<String>(), max_center, max_length, left_idx, right_idx);
    // 3: Find the maximum length and generate answer.
    let answer = &extend_chars[left_idx..right_idx].iter().collect::<String>();
    answer.replace("#", "")
}


fn main() {
    println!("Please input your String ：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read your input !");
    let input_str = input.trim().to_string();
    let result = manacher(input_str);
    println!("Largest palindrome is ：{}", result);
}