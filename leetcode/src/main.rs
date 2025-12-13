use std::cmp::max;

fn main() {

    //running_sum_impl();
    // maximum_wealth_impl();
    length_of_longest_substring_impl();

}

fn running_sum_impl() {
    let nums = vec![3,1,2,10,-1];
    running_sum(nums);
}

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut previous_sum = *nums.get(0).unwrap();
    let mut result_vec = vec![];

    for (i, val) in nums.iter().enumerate() {
        result_vec.push(previous_sum);

        if i+1 < nums.len() {
            previous_sum += nums.get(i+1).unwrap()
        }
    }

    print!("{:?}", result_vec);
    return result_vec;
}

fn maximum_wealth_impl() {
    let accounts: Vec<Vec<i32>> = vec![[2,8,7].to_vec(),[7,1,3].to_vec(),[1,9,5].to_vec()];
    println!("{}", maximum_wealth(accounts));
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        
    let mut max_wealth = 0;

    for acc in accounts.iter() {
        let mut total = 0;
        for balance in acc.iter() {
            total += balance;
        }

        if total > max_wealth {
            max_wealth = total;
        }
    }

    return max_wealth;
}

fn length_of_longest_substring_impl() {
    print!("{}", length_of_longest_substring(String::from("au")));
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length = 0;

    let iter1 = s.chars().into_iter();

    for (i, char) in iter1.enumerate() {
        let mut local_length: i32 = 0;
        let iter2 = s.chars().into_iter().skip(i+1);

        let remaining_length = (s.len() as i32) - 1 - i as i32;
    
        for (_, next_char) in iter2.enumerate() {
            if char == next_char { 
                break;
            }
            local_length += 1;
        }

        if local_length == remaining_length { 
            local_length = 0;
        }


        max_length = max(max_length, local_length+1);
    }


    return max_length;
}


