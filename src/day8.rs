use std::cmp::max;

use array2d::Array2D;

pub fn main_day8() {
    println!("----- DAY 8 --------");

    let test_data = std::fs::read_to_string("./data/day8.txt").unwrap();
    let map: Vec<Vec<usize>> = test_data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let tree_map = Array2D::from_rows(&map).unwrap();
    let n_rows = tree_map.num_rows();
    let n_columns = tree_map.num_columns();

    let mut n_visible: usize = 2 * (n_columns + n_rows) - 4;

    for i in 1..n_rows - 1 {
        for j in 1..n_columns - 1 {
            let row_iter = &mut tree_map.row_iter(i).unwrap();
            let col_iter = &mut tree_map.column_iter(j).unwrap();
            let left_max = row_iter.take(j).max().unwrap();
            let right_max = row_iter.skip(1).max().unwrap();
            let up_max = col_iter.take(i).max().unwrap();
            let down_max = col_iter.skip(1).max().unwrap();

            let val = tree_map.get(i, j).unwrap();
            if val > left_max || val > right_max || val > up_max || val > down_max {
                n_visible += 1;
            }
        }
    }
    println!("Visible {}", n_visible);

    let mut max_score: usize = 0;
    for i in 1..n_rows - 1 {
        for j in 1..n_columns - 1 {
            // unfortunately the library does not yield an exact itertor so we have to collect first
            let row: Vec<&usize> = tree_map.row_iter(i).unwrap().collect();
            let col: Vec<&usize> = tree_map.column_iter(j).unwrap().collect();

            let val = tree_map.get(i, j).unwrap();
            let is_ge = |x: &&usize| *x >= val;

            let left_score = j - &row[..j].into_iter().rposition(is_ge).unwrap_or(0);
            let right_score = 1 + row[j + 1..]
                .into_iter()
                .position(is_ge)
                .unwrap_or(n_columns - j - 2);
            let up_score = i - col[..i].into_iter().rposition(is_ge).unwrap_or(0);
            let down_score = 1 + col[i + 1..]
                .into_iter()
                .position(is_ge)
                .unwrap_or(n_rows - i - 2);
            let score = left_score * right_score * up_score * down_score;
            max_score = max(max_score, score);
        }
    }
    println!("Max score {}", max_score);
}
