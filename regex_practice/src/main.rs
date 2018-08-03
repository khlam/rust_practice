extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./goodfile.txt").expect("Unable to open file");
    println!("\nReject flag: {:?}", check_file("./goodfile.txt").len());
/*
    let f = File::open("./badfile.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut line_num: u8 = 0;
    
    
    for line in f.lines() {
        line_num = line_num + 1;
        /*let question_string = line.unwrap();
        if check_line(&question_string, &line_num) {
            get_study_question(&question_string, &line_num);
        }*/
    }*/
}

fn check_file(file: &str) -> Vec<u8> {
    let f = File::open(&file).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut errors: Vec<u8> = Vec::new();
    let mut line_num: u8 = 0;

    for line in f.lines() {
        line_num = line_num + 1;
        let q_string = line.unwrap();

        if check_comments_or_nl(&q_string, &line_num, &file) {
            let temp = check_question(&q_string, &line_num, &file);
            if temp != 0 {
                errors.push(temp);
            }
        }
    }
    errors
}

#[allow(dead_code)]
fn check_comments_or_nl(line: &String, line_num: &u8, fname: &str) -> bool {
    let line_arr: Vec<char> = line.chars().collect();
    if line_arr.len() == 0 {      // Check for newline
        // println!("LINE\t{:?}\tWARNING: Newline/Empty line", line_num);
        return false;
    }

    if line_arr[0] == '#' {         // Check for comments
        //println!("LINE\t{:?}\t{:?}", line_num, line);
        return false;
    }       
    true
}

fn check_question(line: &String, line_num: &u8, fname: &str) ->u8{
    let line_copy = line.clone();
    let line_parts_iterator = line_copy.split(";");
    let vec: Vec<&str> = line_parts_iterator.collect();

    let field0: String = vec[0].to_string();
    
    let mut temp:u8 = 0;

    if line.chars().filter(|&c| c == ';').count() < 3 {
        println!("\nERROR (1): Missing \';\' delimiter.\n--> {:?}:{:?}\n\t|\n Line {:?} | {:?}\n\t|\n", fname, line_num, line_num, line);
        return 1;
    }

    temp = check_regex(02, "^[0-9]*|^summary$".to_string(), "Invalid entry for step number in Field 0.".to_string(), &field0, &line, &line_num, &fname);
    if temp != 0 { return temp; }

    let field1: String = vec[1].to_string();
    temp = check_regex(03, "^[0-9]*".to_string(), "Invalid entry for question index Field 1.".to_string(), &field1, &line, &line_num, &fname);
    
    let field2: String = vec[2].to_string();
    let f2vec: Vec<&str> = field2.split(":").collect();
    if temp != 0 { return temp; }

    if f2vec.len() > 0 {
        if f2vec[0] == "plain" {
            temp = check_regex(04, "plain:NA:NA".to_string(), "Invalid entry for question type in Field 2. When question type is specified as plain, subfields 1 and 2 must be 'NA'. ".to_string(), &field2, &line, &line_num, &fname);
            if temp != 0 { return temp; }
        }else{
            temp = check_regex(05, "^waitForClick".to_string(), "Invalid entry for question type in Field 2. Question type must be [ plain | waitForClick ]".to_string(), &f2vec[0].to_string(), &line, &line_num, &fname);
            if temp != 0 { return temp; }
            if f2vec.len() > 2  && !f2vec.contains(&""){
                temp = check_regex(06, "^(gameboard_?|rewardBar_?|saliencyMap_?){0,3}$".to_string(), "Invalid entry for question type in Field 2, subfield 1. Valid entries are {gameboard, rewardBar, saliencyMap}, deliminated by _.".to_string(), &f2vec[1].to_string(), &line, &line_num, &fname);
                if temp != 0 { return temp; }
            }else{
                temp = check_regex(07, "$^".to_string(), "Invalid entry for question type in Field 2. Question type waitForClick cannot have blank subfields.".to_string(), &field2, &line, &line_num, &fname);
                if temp != 0 { return temp; }
            }
        }
    }else {
        temp = check_regex(08, "$^".to_string(), "Invalid entry for question type in Field 2.".to_string(), &field2, &line, &line_num, &fname);
        if temp != 0 { return temp; }
    }

    return 0;
}

/*
check_regex args:
1. error code (0 means no error)
2. regex expression, if string does not pass it then this is an error
3. message explaining what the error is
4. reference to the substring to check
5. the full parent string
6. line number the substring is from
7. the filename the substring is from 
*/
fn check_regex(error_code: u8, regex: String, msg: String, str_to_check: &String, full_line: &String, line_num: &u8, fname: &str) -> u8 {
    let step_reg = Regex::new(&regex).unwrap();
    if !step_reg.is_match(&str_to_check) {
        println!("\nERROR ({}): {}\n\t--> {}:{}\n\t|\n Line {} | {}\n\t|\n\tInvalid substring: {:?}\n", error_code, msg, fname, line_num, line_num, full_line, str_to_check);
        return error_code;
    }
    0
}
