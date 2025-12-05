use std::fs;

fn main() {
    let file_path = "src/input.txt";

    let part_one_res = run_part_one(file_path);

    println!("Part one results: {:?}", part_one_res);
}

fn find_index(input: &Vec<i32>, sorted_input: &Vec<i32>) -> usize {
    let index = input.iter().position(|&r| r == sorted_input[0]).unwrap();

    index
}

fn run_part_one(fp: &str) -> i32 {
    let mut res_matrix: Vec<i32> = vec![];

    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");

    for line in contents.lines() {
        let chars: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        res_matrix.push(find_best_joltage(chars));
    }

    let final_sum: i32 = res_matrix.iter().sum();
    final_sum
}

// Base 10 means that we want to maximze the first digit first. So we can
// get the largest value and find its index. If it is last in the list, then
// we go onto the second largest; otherwise, we test all values after its
// index (since we can't rearrange the values).
// If there's more than one, we should find the earlier (since them being paired together would be the largest value).
fn find_largest_value_index(input_bank: &Vec<i32>) -> usize {
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

    if index == input_bank.len() - 1 {
        // println!("Zeroing first index");
        sorted_input.remove(0);
        index = find_index(&input_bank, &sorted_input);
    }

    // println!("Value of index key: {}", input_bank[index]);

    index
}

fn covert_indices_to_digits(tens: i32, ones: i32) -> i32 {
    let tens_str = tens.to_string();
    let ones_str = ones.to_string();

    let new_value = (tens_str + &ones_str).parse().unwrap();

    new_value
}

fn find_best_joltage(input_bank: Vec<i32>) -> i32 {
    // We have the best 10s place, so we can optimize with all values in the following indices and compare them:
    let best_index = find_largest_value_index(&input_bank);
    let best_value = &input_bank[best_index];

    let ones_options = &input_bank[best_index + 1..].to_vec();

    let values: Vec<i32> = ones_options
        .iter()
        .map(|&x| covert_indices_to_digits(*best_value, x))
        .collect();

    println!("{:?}", values);
    *values.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_find_best_joltage() {
    //     let input: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
    //     let want: i32 = 98;

    //     let res = find_best_joltage(input);

    //     assert_eq!(want, res);
    // }

    #[test]
    fn test_find_largest_value_index() {
        let input: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        let want: usize = 0;
        let res = find_largest_value_index(&input);

        assert_eq!(want, res);
    }

    #[test]
    fn test_find_largest_recursive() {
        let input2: Vec<i32> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let res2 = find_largest_value_index(&input2);
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
        let input2: Vec<i32> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let res2 = find_best_joltage(input2);
        assert_eq!(want, res2);
    }

    #[test]
    fn test_run_part_one() {
        let want = 357;
        let res = run_part_one("src/example.txt");
        assert_eq!(want, res);
    }
}
