/**** 
 * levenshtein distance is a distance to evaluate the cost from straing A change to string B. There are 3 different cost :
 *  1. delete : delate a char in A, naming as cost_delete, by default is 1
 *  2. add  : add a char in A, naming as cost_add, by default is 1
 *  3. replace : replace a char in A from the same postion of string B, naming as cost_replace, by default is 1.
 * The main idea is to compare 3 different cost from the matrix length_A+1, length_B+1, for each index (i,j). Here the word start with index 1.
 *  i-1, j --> i,j means add
 *  i, j-1 --> i,j means delete
 *  i-1, j-1 --> i,j means replace
 * ****/

/****Test case
 * hourse 
 * ros
 * -> 3
 * 
 * intention
 * execution
 * -> 5
 * 
 * 今天天气不错
 * 天气好不好
 * -> 4
 * 
 *****/


 fn levenshtein_distance(word1: String, word2: String, cost_add:usize, cost_delete:usize, cost_replace:usize) -> usize {
    let m = word1.chars().count();
    let n = word2.chars().count();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cur_cost_replace = if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) { 0 } else { cost_replace };
            // i-1, j --> delete, i, j-1 --> add,  i-1, j-1 --> replace.
            dp[i][j] = std::cmp::min(
                dp[i - 1][j] + cost_delete,
                std::cmp::min(
                    dp[i][j - 1] + cost_add,
                    dp[i - 1][j - 1] + cur_cost_replace,
                ),
            );
        }
    }
    dp[m][n]
}

fn main() {
    let cost_add:usize = 1;
    let cost_delete:usize = 1;
    let cost_replace:usize = 1;
    
    println!("Please Type word 1 : ");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Couldn't read your input !");
    let input1_str = input1.trim().to_string();
        
    println!("Please Type word 2 : ");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Couldn't read your input !");
    let input2_str = input2.trim().to_string();
        
    let result = levenshtein_distance(input1_str, input2_str, cost_add, cost_delete, cost_replace);
    println!("Edit distance is : {}  (cost_add : {}, cost_delete: {}, cost_replace: {})", result, cost_add, cost_delete, cost_replace);
}