use grid::Grid; // 用于 lcs()
use std::env;
use std::fs::File; // 用于 read_file_lines()
use std::io::{self, BufRead, BufReader}; // 用于 read_file_lines()
use std::process;

pub mod grid;

/// 读取给定路径下的文件，并返回一个字符串向量。
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut vec:Vec<String> = Vec::new();

    for line in reader.lines() {
        let line1 = line?;
        vec.push(line1);
    }
    Ok(vec)
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    let m = seq1.len();
    let n = seq2.len();

    let mut dp = Grid::new(m + 1, n + 1);

    for i in 0..=m {
        dp.set(i, 0, 0).unwrap();
    }

    for j in 0..=n {
        dp.set(0, j, 0).unwrap();
    }

    for i in 1..=m {
        for j in 1..=n {
            if seq1[i - 1] == seq2[j - 1] {
                let val = dp.get(i - 1, j - 1).unwrap_or(0) + 1;
                dp.set(i, j, val).unwrap();
            } else {
                let val1 = dp.get(i, j - 1).unwrap_or(0);
                let val2 = dp.get(i - 1, j).unwrap_or(0);
                let max_val = std::cmp::max(val1, val2);
                dp.set(i, j, max_val).unwrap();
            }
        }
    }

    dp
}


fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && lines1[i-1] == lines2[j-1] {
        print_diff(lcs_table, lines1, lines2, i-1, j-1);
        println!("  {}", lines1[i-1]);
    } else if j > 0 && (i == 0 || lcs_table.get(i, j-1).unwrap_or(0) >= lcs_table.get(i-1, j).unwrap_or(0)) {
        print_diff(lcs_table, lines1, lines2, i, j-1);
        println!("> {}", lines2[j-1]);
    } else if i > 0 && (j == 0 || lcs_table.get(i, j-1).unwrap_or(0) < lcs_table.get(i-1, j).unwrap_or(0)) {
        print_diff(lcs_table, lines1, lines2, i-1, j);
        println!("< {}", lines1[i-1]);
    } else {
        println!("");
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("参数太少。");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let result1 = read_file_lines(filename1).expect("无法读取文件1");
    let result2 = read_file_lines(filename2).expect("无法读取文件2");

    let lcs_table: Grid = lcs(&result1, &result2);

    print_diff(&lcs_table, &result1, &result2, result1.len(), result2.len());
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
