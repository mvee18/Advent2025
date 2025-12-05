use std::fs;

fn main() {
    let fp: &str = "day02/src/input.txt";
    let inputs: Vec<String> = read_input(fp);
    let res = sum_int_vec(check_input_sequences(inputs));
    println!("Part 1 Result: {}", res);
    let res2 = sum_int_vec(check_any_repeats_inputs(read_input(fp)));
    println!("Part 2 Result: {}", res2);
}

fn read_input(fp: &str) -> Vec<String> {
    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");

    // Split contents on ,
    let inputs: Vec<String> = contents.split(",").map(|s| s.to_string()).collect();

    inputs
}

fn parse_id(input_str: &str) -> Vec<i64> {
    // Each string looks like this "xxx-yyy" with no leading zeroes. Any string with repeat is invalid.
    let splitted: Vec<i64> = input_str
        .split("-")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // println!("{:?}", splitted);
    splitted
}

fn check_if_repeat_in_value(input_int: i64) -> i64 {
    // The maximum size of a repeat is the floor of len(input)/2, else it wouldn't fit in the value more than once. This reduces the values we need to check.
    // This also means odd value ints can't repeat exactly in half, which makes mathematical sense. So we can immediately filter out odd values as not possible repeats.
    let input_len: usize = input_int.to_string().len();

    if input_len % 2 == 1 {
        // Odd length values can't repeat twice.
        return 0;
    }

    // Midpoint
    let max_repeat_index: usize = input_len / 2;
    // println!(
    //     "Repeated index {:?}",
    //     &input_int.to_string()[0..max_repeat_index]
    // );

    // Now we can check if left and right are equal:
    let left: &str = &input_int.to_string()[0..max_repeat_index];
    let right: &str = &input_int.to_string()[max_repeat_index..];

    // println!("Left: {:?}, Right: {:?}", left, right);

    if left == right {
        return input_int;
    } else {
        return 0;
    }
}

// ID is invalid if it is made only of some sequence of digits repeated *at least* twice.
fn check_if_any_repeats(input: i64) -> i64 {
    // Easiest way is to start with the first value, then see if the following value is the same. Then two, and check if the following two are the same, etc.
    // The value has to be all repeats though, so we can't just check the first repeat.
    let input_str: String = input.to_string();

    // I've changed my mind, let's do it cascading. First, check if the first digit is the same as the second. If it is, check the rest of the string in chunks of that size.
    let input_len: usize = input_str.len();
    for repeat_size in 1..=input_len / 2 {
        // Only check if the input length is divisible by the repeat size.
        if input_len % repeat_size != 0 {
            continue;
        }

        let first_chunk: &str = &input_str[0..repeat_size];
        let mut all_match: bool = true;

        // Now check each chunk of size repeat_size
        for start_index in (0..input_len).step_by(repeat_size) {
            let end_index = start_index + repeat_size;
            let current_chunk: &str = &input_str[start_index..end_index];

            if current_chunk != first_chunk {
                all_match = false;
                break;
            }
        }

        if all_match {
            return input;
        }
    }

    return 0;
}

// Takes the input_ids and returns a vector with all values in the sequence between them.
fn expand_range(input_vec: Vec<i64>) -> Vec<i64> {
    let res: Vec<i64> = (input_vec[0]..=input_vec[1]).collect();
    res
}

fn sum_int_vec(input_vecvec: Vec<Vec<i64>>) -> i64 {
    let res: i64 = input_vecvec.iter().flatten().sum();
    println!("Sum: {}", res);

    res
}

fn check_input_sequences(inputs: Vec<String>) -> Vec<Vec<i64>> {
    // Over each input parse the id into a Vec<i64> then expand the range. Collect the results into a vec of vecs of i64s.
    let inputs_int: Vec<Vec<i64>> = inputs.iter().map(|s| expand_range(parse_id(s))).collect();

    // println!("inputs int: {:?}", inputs_int);

    // Now for each list in the list of lists, we can detect if it has a repeat.
    let res_int: Vec<Vec<i64>> = inputs_int
        .iter()
        .map(|inner_vec| {
            inner_vec
                .iter()
                .map(|&x| check_if_repeat_in_value(x))
                .collect()
        })
        .collect();

    // println!("{:?}", res_int);

    res_int
}

fn check_any_repeats_inputs(inputs: Vec<String>) -> Vec<Vec<i64>> {
    // Over each input parse the id into a Vec<i64> then expand the range. Collect the results into a vec of vecs of i64s.
    let inputs_int: Vec<Vec<i64>> = inputs.iter().map(|s| expand_range(parse_id(s))).collect();

    // println!("inputs int: {:?}", inputs_int);

    // Now for each list in the list of lists, we can detect if it has a repeat.
    let res_int: Vec<Vec<i64>> = inputs_int
        .iter()
        .map(|inner_vec| inner_vec.iter().map(|&x| check_if_any_repeats(x)).collect())
        .collect();

    // println!("{:?}", res_int);

    res_int
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_parse_id() {
        let want: Vec<i64> = vec![11, 22];
        let res = parse_id("11-22");
        assert_eq!(res, want)
    }

    #[test]
    fn test_check_repeat_i() {
        let want: i64 = 11;
        let res = check_if_repeat_in_value(11);
        assert_eq!(want, res)
    }

    #[test]
    fn test_check_repeat_ii() {
        let want: i64 = 1188511885;
        let res = check_if_repeat_in_value(1188511885);
        assert_eq!(want, res)
    }

    #[test]
    fn test_expand_range() {
        let want: Vec<i64> = vec![11, 12, 13, 14, 15];
        let res = expand_range(vec![11, 15]);
        assert_eq!(want, res)
    }

    #[test]
    fn test_check_input_sequences() {
        let input: Vec<String> = vec!["11-13".to_string()];
        let want: Vec<Vec<i64>> = vec![vec![11, 0, 0]];
        let res = check_input_sequences(input);

        assert_eq!(want, res)
    }

    #[test]
    fn test_example_sum() {
        let fp: &str = "src/example.txt";
        let inputs: Vec<String> = read_input(fp);
        let res = sum_int_vec(check_input_sequences(inputs));

        assert_eq!(res, 1227775554);
    }

    #[test]
    fn test_if_any_repeats_in_int() {
        let input1: i64 = 824824824;
        let input2: i64 = 2121212121;
        let input3: i64 = 11;

        let want1: i64 = 824824824;
        let want2: i64 = 2121212121;
        let want3: i64 = 11;

        let res1 = check_if_any_repeats(input1);
        let res2 = check_if_any_repeats(input2);
        let res3 = check_if_any_repeats(input3);

        assert_eq!(res1, want1);
        assert_eq!(res2, want2);
        assert_eq!(res3, want3);
    }

    #[test]
    fn test_part_ii() {
        let fp: &str = "src/example.txt";
        let inputs: Vec<String> = read_input(fp);
        let res = sum_int_vec(check_any_repeats_inputs(inputs));

        assert_eq!(res, 4174379265);
    }
}
