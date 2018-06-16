extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //let f = File::open("./goodfile.txt").expect("Unable to open file");
    let f = File::open("./badfile.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut line_num: u8 = 0;
    
    for line in f.lines() {
        line_num = line_num + 1;
        let question_string = line.unwrap();
        if check_line(&question_string, &line_num) {
            //println!("\n{:?}", question_string);
        }
    }
}

fn check_line(line: &String, line_num: &u8) -> bool {
    let line_arr: Vec<char> = line.chars().collect();
    if line_arr.len() == 0 {      // Check for newline
        println!("LINE\t{:?}\tWARNING: Newline/Empty line", line_num);
        return false;
    }

    if line_arr[0] == '#' {         // Check for comments
        println!("LINE\t{:?}\t{:?}", line_num, line);
        return false;
    }

    if line.chars().filter(|&c| c == ';').count() < 3 {
        panic!("\n\tERROR: line {:?}\t Missing \';\' delimiter.", line_num);
    }

    // Match Summary Regex
    let summary_re = Regex::new("^([0-9]+|summary);[0-9]+;plain:NA:NA;").unwrap();
    if summary_re.is_match(line) {
        println!("LINE\t{:?}\tSUMMARY REGEX MATCH", line_num);
    } else{
        // Wait For Click Regex
        let wait_for_click_re = Regex::new("^[0-9]+;[0-9]+;waitForClick:(gameboard_|rewardBar_|saliencyMap_)*(gameboard_|rewardBar_|saliencyMap_)*(gameboard:|rewardBar:|saliencyMap:)+.+;").unwrap();
        if (wait_for_click_re.is_match(line)){
            println!("LINE\t{:?}\tWAIT FOR CLICK REGEX MATCH", line_num);
        } else{
            println!("LINE\t{:?}\tREGEX MISMATCH", line_num);
        }
    }        

    
    true
}