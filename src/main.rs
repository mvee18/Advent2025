use core::time;
use std::fs;

fn main() {
    let file_path = "src/input.txt";
    let mut starting_dial: i32 = 50;

    let zeroes = perform_code(file_path, &mut starting_dial);
    println!("Exact zeroes: {}", zeroes.0);
    println!("Total zeroes: {}", zeroes.1);
}

fn perform_code(file_path: &str, starting_dial: &mut i32) -> (i32, i32) {
    let mut zeroes = 0;
    let mut total_zeroes = 0;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        let (res, times_cross_zero) = rotate_dial(starting_dial, line);
        println!("{}: {}, {}", line, res, times_cross_zero);
        total_zeroes += times_cross_zero;
        if res == 0 {
            zeroes += 1;
            total_zeroes += 1;
        }
    }

    (zeroes, total_zeroes)
}

fn parse_rotation(rotation: &str) -> i32 {
    let split: Vec<char> = rotation.chars().collect();

    let mut direction = 1;
    if split[0] == 'L' {
        direction *= -1
    } else {
        direction *= 1
    }

    // Collect everything besides the first character...
    let nums = &split[1..];

    // Convert to string then parse to i32
    let magnitude_str: String = nums.iter().collect();
    let magnitude: i32 = magnitude_str.parse().unwrap();

    return direction * magnitude;
}

// The dial starts at 50, then rotates L{XX} or R{XX}.
// However, the dial wraps around at 0 and 99.
// Use L as negative and R as positive.
pub fn rotate_dial(current_pos: &mut i32, rotation: &str) -> (i32, i32) {
    let rotation_result = parse_rotation(rotation);
    let mut times_crossed_zero = 0;

    let mut final_position = *current_pos + rotation_result;

    println!("Starting position: {}", current_pos);
    println!("Rotation operation: {}", rotation);
    println!("Final position before mod: {}", final_position);
    times_crossed_zero += calculate_times_crossed_zero(*current_pos, final_position);

    // Rotate it the other way if it is negative...
    if final_position <= 0 {
        final_position += 100;
        final_position = final_position % 100
    } else {
        // Otherwise, catch the edge cases of landing on 100
        final_position = final_position % 100
    }

    // The pointer needs to be updated so the next loop works...
    *current_pos = final_position;
    return (final_position, times_crossed_zero);
}

fn calculate_from_sequence(start: i32, end: i32) -> i32 {
    let mut dial_sequence: Vec<i32> = (start..end).collect();
    // The first element can be 0 -- this leads to off by one error. We can just drop it...
    dial_sequence.remove(0);
    let mod_seq: Vec<i32> = dial_sequence.into_iter().map(|x| x % 100).collect();
    let zeroes: i32 = mod_seq.iter().filter(|&x| *x == 0).count() as i32;

    zeroes
}

// Make a sequence from the starting to end and count the times where mod 100 == 0
fn calculate_times_crossed_zero(starting_pos: i32, final_position: i32) -> i32 {
    let mut times_crossed_zero = 0;

    if starting_pos < final_position {
        times_crossed_zero += calculate_from_sequence(starting_pos, final_position);
    } else {
        times_crossed_zero += calculate_from_sequence(final_position, starting_pos);
    }

    times_crossed_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_passes() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_rotate_dial_positive() {
        let mut starting_pos = 52;
        let pos_res = rotate_dial(&mut starting_pos, "R48").0;
        assert_eq!(pos_res, 0);

        assert_eq!(starting_pos, 0)
    }

    #[test]
    fn test_rotate_dial_negative() {
        let mut starting_pos = 50;
        assert_eq!(rotate_dial(&mut starting_pos, "L68").0, 82);
        assert_eq!(starting_pos, 82)
    }

    #[test]
    fn test_parse_small_rotation() {
        assert_eq!(parse_rotation("L5"), -5)
    }

    #[test]
    fn test_exact_zeroes() {
        let mut starting_pos = 50;
        assert_eq!(perform_code("src/example1.txt", &mut starting_pos).0, 3);
    }

    #[test]
    fn test_parse_large_rotation() {
        assert_eq!(parse_rotation("R1000"), 1000)
    }

    #[test]
    fn test_large_rotation() {
        let mut starting_pos = 50;
        assert_eq!(rotate_dial(&mut starting_pos, "R1000").0, 50);
    }

    #[test]
    fn test_pass_zero_1() {
        assert_eq!(rotate_dial(&mut 50, "L68").1, 1)
    }

    #[test]
    fn test_pass_zero_2() {
        assert_eq!(rotate_dial(&mut 50, "L68").1, 1)
    }
    #[test]
    fn test_pass_zero_3() {
        assert_eq!(rotate_dial(&mut 82, "L30").1, 0)
    }
    #[test]
    fn test_pass_zero_4() {
        assert_eq!(rotate_dial(&mut 52, "R48").1, 0)
    }
    #[test]
    fn test_pass_zero_5() {
        assert_eq!(rotate_dial(&mut 0, "L5").1, 0)
    }
    #[test]
    fn test_pass_zero_6() {
        assert_eq!(rotate_dial(&mut 95, "R60").1, 1)
    }
    #[test]
    fn test_pass_zero_7() {
        assert_eq!(rotate_dial(&mut 55, "L55").1, 0)
    }
    #[test]
    fn test_pass_zero_8() {
        assert_eq!(rotate_dial(&mut 0, "L1").1, 0)
    }
    #[test]
    fn test_pass_zero_9() {
        assert_eq!(rotate_dial(&mut 99, "L99").1, 0)
    }
    #[test]
    fn test_pass_zero_10() {
        assert_eq!(rotate_dial(&mut 0, "R14").1, 0)
    }
    #[test]
    fn test_pass_zero_11() {
        assert_eq!(rotate_dial(&mut 14, "L82").1, 1)
    }

    #[test]
    fn integrate_test_example() {
        assert_eq!(perform_code("src/example1.txt", &mut 50).1, 6)
    }
}

// The dial starts by pointing at 50.
// The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
// The dial is rotated L30 to point at 52.
// The dial is rotated R48 to point at 0.
// The dial is rotated L5 to point at 95.
// The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
// The dial is rotated L55 to point at 0.
// The dial is rotated L1 to point at 99.
// The dial is rotated L99 to point at 0.
// The dial is rotated R14 to point at 14.
// The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
