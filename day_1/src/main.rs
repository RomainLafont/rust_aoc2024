use std::env;
use std::fs;
use std::str::FromStr;

fn extract_data_from_file(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut input1: Vec<i32> = Vec::new();
    let mut input2: Vec<i32> = Vec::new();
    for line in fs::read_to_string(&filename).unwrap().lines() {
        let (entry1, entry2) = match line.find(' ') {
            None => None,
            Some(index) => {
                match (i32::from_str(&line[..index]), i32::from_str(&line[index + 3..])) {
                    (Ok(l), Ok(r)) => Some((l, r)),
                    _ => None
                }
            }
        }.unwrap();
        input1.push(entry1);
        input2.push(entry2);
    }
    (input1, input2)
}

fn calculate_similarity_score(input1: &Vec<i32>, input2: &Vec<i32>) -> i32 {
    let mut score = 0;
    for element in input1 {
        let similarity = input2.iter().filter(|&n| *n == *element).count() as i32 * element;
        score = score + similarity;
    }
    score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let (mut input1, mut input2) = extract_data_from_file(&filename);

    input1.sort();
    input2.sort();
    let abs_difference = input1.clone().into_iter().zip(input2.clone()).map(|(a, b)| (a - b).abs()).collect::<Vec<i32>>();
    let sum_distance : i32 = abs_difference.iter().sum();
    println!("Sum distance = {}", sum_distance);

    let similarity_score = calculate_similarity_score(&input1, &input2);
    println!("Similarity score = {}", similarity_score);
}
