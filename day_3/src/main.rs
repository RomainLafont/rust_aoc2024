use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let regex = Regex::new(r#"(mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\)))"#).unwrap();
    let mut product : u32 = 0;
    let mut should_do = 1;
    for line in fs::read_to_string(&filename).unwrap().lines() {
        let factor_couples : Vec<(&str, &str, &str, &str)> = regex.captures_iter(line).map(|caps| {
            let x = match caps.name("x") {
                Some(v) => v.as_str(),
                None => "0"
            };
            let y = match caps.name("y") {
                Some(v) => v.as_str(),
                None => "0"
            };
            // caps.name("y").unwrap().as_str();
            let do_ = match caps.name("do") {
                Some(_v) => "1",
                None => "0"
            };
            // caps.name("do").unwrap().as_str();
            let dont = match caps.name("dont") {
                Some(_v) => "1",
                None => "0"
            };
            // caps.name("dont").unwrap().as_str();
            (x, y, do_, dont)
        }).collect();
        println!("{:?}", factor_couples);
        for factor_couple in factor_couples {
            if should_do == 1 {
                product += factor_couple.0.parse::<u32>().unwrap() * factor_couple.1.parse::<u32>().unwrap();
                if factor_couple.3.parse::<u32>().unwrap() == 1 {
                    should_do = 0;
                }
            } else {
                if factor_couple.2.parse::<u32>().unwrap() == 1 {
                    should_do = 1;
                }
            }
        }
        //product += factor_couples.iter().map(|factors| (factors.0.parse::<u32>().unwrap()) * (factors.1.parse::<u32>().unwrap())).collect::<Vec<u32>>().into_iter().sum::<u32>();
        println!("Product = {}", product);
    }
}
