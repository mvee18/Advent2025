use std::fs;

// TRUE == Another Roll of Paper Present
// FALSE == CLEAR

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    v: char,
}

impl Point {
    fn check_up(&self, m: Matrix) -> Option<bool> {
        // I have self.x and self.y
        // Up == subtract one from y
        if self.y == 0 {
            Some(false)
        } else {
            let up_y = (self.y - 1);
            let new_point = m.get(self.x, up_y).unwrap().v;

            if new_point == '.' {
                Some(false)
            } else {
                Some(true)
            }
        }
    }
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<Point>>,
}

impl Matrix {
    fn get(&self, x: i32, y: i32) -> Option<&Point> {
        if y < 0 || y as usize >= self.data.len() {
            return None;
        }
        if x < 0 || x as usize >= self.data[y as usize].len() {
            return None;
        }
        Some(&self.data[y as usize][x as usize])
    }

    fn pprint(&self) {
        for row in &self.data {
            for point in row {
                print!("{}", point.v);
            }
            println!();
        }
    }
}

fn main() {
    let res = read_input("src/example.txt");
    println!("{:?}", res);
    println!("{:?}", res.get(0, 3).unwrap());
    res.pprint();
}

fn read_input(fp: &str) -> Matrix {
    let contents = fs::read_to_string(fp).expect("Should have been able to read the file");

    let mut res_matrix: Matrix = Matrix { data: Vec::new() };

    for (i, line) in contents.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();

        for (j, c) in chars.iter().enumerate() {
            if res_matrix.data.len() <= i {
                res_matrix.data.push(Vec::new());
            }
            res_matrix.data[i].push(Point {
                x: j as i32,
                y: i as i32,
                v: *c,
            });
        }
    }

    res_matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    let 

    #[test]
    fn test_check_up() {
      let test_mat = Matrix()
    }
}
