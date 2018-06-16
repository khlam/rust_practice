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
            get_study_question(&question_string, &line_num);
        }
    }
}

fn check_line(line: &String, line_num: &u8) -> bool {
    let line_arr: Vec<char> = line.chars().collect();
    if line_arr.len() == 0 {      // Check for newline
        // println!("LINE\t{:?}\tWARNING: Newline/Empty line", line_num);
        return false;
    }

    if line_arr[0] == '#' {         // Check for comments
        println!("LINE\t{:?}\t{:?}", line_num, line);
        return false;
    }

    if line.chars().filter(|&c| c == ';').count() < 3 {
        panic!("\n\tERROR: line {:?}\t Missing \';\' delimiter.", line_num);
    }       
    true
}

fn get_study_question(line: &String, line_num: &u8) {
    let line_parts_iterator = line.split(";");
    let vec: Vec<&str> = line_parts_iterator.collect();

    let step: String = vec[0].to_string();
    check_regex("^[0-9]+$|^summary$".to_string(), "STEP REGEX FAIL".to_string(), &step, &line_num);

    let question_index: String = vec[1].to_string();
    check_regex("^[0-9]+$".to_string(), "INDEX REGEX FAIL".to_string(), &question_index, &line_num);
    
    let question_type: String = vec[2].to_string();
    check_regex("plain:NA:NA|^waitForClick:(gameboard_|rewardBar_|saliencyMap_){0, 2}(gameboard|rewardBar|saliencyMap){0, 1}:.+".to_string(), "QUESTION TYPE REGEX FAIL".to_string(), &question_type, &line_num);

    let question : String = vec[3].to_string();
    
    let mut answer_vec : Vec<String> = Vec::new();
    for x in 4..vec.len() {
        answer_vec.push(vec[x].to_string());
    }
}

fn check_regex(regex: String, msg: String, line: &String, line_num: &u8) {
    let step_reg = Regex::new(&regex).unwrap();
    if !step_reg.is_match(&line) {                      // on Regex no match, panic
        println!("LINE\t{:?}\t{:?}", line_num, msg);
       // return true;
    }
}