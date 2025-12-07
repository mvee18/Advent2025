use std::fs;

fn main() {
    let file_path = "src/input.txt";

    let part_one_res = run_joltage(file_path, 2);
    let part_two_res = run_joltage(file_path, 12);

    println!("Part one results: {:?}", part_one_res);
    println!("Part two results: {:?}", part_two_res);
}

fn find_index(input: &Vec<i64>, sorted_input: &Vec<i64>) -> usize {
    // println!("Printing input: {:?}", input);
    let index = input.iter().position(|&r| r == sorted_input[0]).unwrap();

    index
}

fn run_joltage(fp: &str, desired_size: i64) -> i64 {
    let mut res_matrix: Vec<i64> = vec![];

    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");

    for line in contents.lines() {
        let chars: Vec<i64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        res_matrix.push(find_best_joltage_recursive(&chars, desired_size));
    }

    // println!("Resulting matrix: {:?}", res_matrix);

    let final_sum: i64 = res_matrix.iter().sum();
    final_sum
}

// Base 10 means that we want to maximze the first digit first. So we can
// get the largest value and find its index. If it is last in the list, then
// we go onto the second largest; otherwise, we test all values after its
// index (since we can't rearrange the values).
// If there's more than one, we should find the earlier (since them being paired together would be the largest value).
fn find_largest_value_index(input_bank: &Vec<i64>, desired_size: i64) -> usize {
    // Clone it; we don't want to change the original orientation.
    let mut sorted_input = input_bank.clone();
    sorted_input.sort_by(|a, b| b.cmp(a));

    // println!("{:?}", sorted_input);

    // let mut index = input_bank
    //     .iter()
    //     .position(|&r| r == sorted_input[0])
    //     .unwrap();

    let mut index = find_index(&input_bank, &sorted_input);

    // Sometimes, the largest value is at the end; therefore, we can pop the first index from sorted_input, then recursively run the find_largest_value_index.
    // println!("Current index {}", index);
    // println!("Length of input: {}", input_bank.len());

    // We can use "desired size" to determine the maximum index of the desired place. So for example, if we want a two digit (tens and ones), then the max index is len - 1 (since it can't be the last index).

    // Apply this logic to desired size: the index must be less than len - desired_size.

    while index > input_bank.len() - desired_size as usize {
        // println!("Zeroing first index");
        sorted_input.remove(0);
        index = find_index(&input_bank, &sorted_input);
        // println!("New index {}", index);
    }

    // println!("Value of index key: {}", input_bank[index]);

    index
}

fn covert_indices_to_digits(tens: i64, ones: i64) -> i64 {
    let tens_str = tens.to_string();
    let ones_str = ones.to_string();

    let new_value = (tens_str + &ones_str).parse().unwrap();

    new_value
}

fn join_n_digits(digits: Vec<i64>) -> i64 {
    let res_str: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
    let joined_str = res_str.join("");
    let res: i64 = joined_str.parse().unwrap();

    res
}

fn find_best_joltage(input_bank: Vec<i64>) -> i64 {
    // We have the best x's place, so we can optimize with all values in the following indices and compare them:
    let best_index = find_largest_value_index(&input_bank, 2);
    let best_value = &input_bank[best_index];

    let ones_options = &input_bank[best_index + 1..].to_vec();

    let values: Vec<i64> = ones_options
        .iter()
        .map(|&x| covert_indices_to_digits(*best_value, x))
        .collect();

    println!("{:?}", values);
    *values.iter().max().unwrap()
}

fn find_best_joltage_recursive(input_bank: &Vec<i64>, desired_size: i64) -> i64 {
    // If find_largest_values_index can find the best candidate at any index, we can recursively call this function to find the next best candidate but we will have to pass in the current best index to limit the search space.

    let mut input_clone = input_bank.clone();
    let mut best_index = find_largest_value_index(&input_bank, desired_size);

    // println!("Best index found: {}", best_index);

    let mut res: Vec<i64> = vec![input_clone[best_index]];
    // So we can loop this function until we reach the end of the list.
    for i in 1..desired_size {
        input_clone = input_clone[best_index + 1..].to_vec();

        // println!("New slice: {:?}", input_clone);

        best_index = find_largest_value_index(&input_clone, desired_size - i);

        // println!("Best index found: {}", best_index);
        // println!("Value at best index: {}", input_clone[best_index]);

        res.push(input_clone[best_index]);
    }

    // println!("Resulting indices: {:?}", res);

    join_n_digits(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_find_best_joltage() {
    //     let input: Vec<i64> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
    //     let want: i64 = 98;

    //     let res = find_best_joltage(input);

    //     assert_eq!(want, res);
    // }

    #[test]
    fn test_find_largest_value_index() {
        let input: Vec<i64> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        let want: usize = 0;
        let res = find_largest_value_index(&input, 2);

        assert_eq!(want, res);
    }

    #[test]
    fn test_find_largest_recursive() {
        let input2: Vec<i64> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let res2 = find_largest_value_index(&input2, 2);
        assert_eq!(13, res2);
    }

    #[test]
    fn test_convert_digits() {
        let tens = 5;
        let ones = 1;

        let res = covert_indices_to_digits(tens, ones);

        assert_eq!(51, res);
    }

    #[test]
    fn test_find_best_joltage() {
        let want = 78;
        let input2: Vec<i64> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let res2 = find_best_joltage(input2);
        assert_eq!(want, res2);
    }

    #[test]
    fn test_run_part_one() {
        let want = 357;
        let res = run_joltage("src/example.txt", 2);
        assert_eq!(want, res);
    }

    #[test]
    fn test_find_largest_value_index_12_digits() {
        let want = 2;
        let input: Vec<i64> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let res = find_largest_value_index(&input, 12);
        assert_eq!(want, res);
    }

    #[test]
    fn test_find_best_joltage_recursive() {
        let want = 784;
        let input2: Vec<i64> = vec![2, 3, 4, 1, 7, 8, 4];
        let res2 = find_best_joltage_recursive(&input2, 3);
        assert_eq!(want, res2);
    }

    #[test]
    fn test_join_n_digits() {
        let digits = vec![7, 8, 4];
        let res = join_n_digits(digits);
        assert_eq!(784, res);
    }

    #[test]
    fn test_part_ii() {
        let want: i64 = 3121910778619;
        let res = run_joltage("src/example.txt", 12);
        assert_eq!(want, res);
    }
}
