use std::fs;

fn count_increase(data: Vec<i32>) -> usize {
    let mut count = 0;
    for i in 0..data.len() - 1 {
        if data[i] < data[i + 1] {
            count += 1;
        }
    }
    count
}

fn sliding_window_sum(data: Vec<i32>, size: usize) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..data.len() - size + 1 {
        let mut sum = 0;
        for j in i..i + size {
            sum += data[j];
        }
        result.push(sum);
    }
    result
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let data: Vec<i32> = content
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let result = sliding_window_sum(data, 3);
    let count = count_increase(result);
    println!("{:?}", count);
}
