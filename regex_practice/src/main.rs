extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //let f = File::open("./goodfile.txt").expect("Unable to open file");
    println!("\nReject flag: {:?}", check_file("./badfile.txt"));
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

fn check_file(file: &str) -> bool{
    let f = File::open(&file).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut line_num: u8 = 0;
    let mut reject_file = false;

    for line in f.lines() {
        line_num = line_num + 1;
        let q_string = line.unwrap();

        if check_line(&q_string, &line_num, &file) {
            check_question(&q_string, &line_num, &file);
        }
    }
    reject_file
}

fn check_line(line: &String, line_num: &u8, fname: &str) -> bool {
    let line_arr: Vec<char> = line.chars().collect();
    if line_arr.len() == 0 {      // Check for newline
        // println!("LINE\t{:?}\tWARNING: Newline/Empty line", line_num);
        return false;
    }

    if line_arr[0] == '#' {         // Check for comments
        //println!("LINE\t{:?}\t{:?}", line_num, line);
        return false;
    }

    if line.chars().filter(|&c| c == ';').count() < 3 {
        println!("\nERROR: Missing \';\' delimiter.\n--> {:?}:{:?}\n\t|\n Line {:?} | {:?}\n\t|\n", fname, line_num, line_num, line);
        return false;
    }       
    true
}

fn check_question(line: &String, line_num: &u8, fname: &str) {
    let line_copy = line.clone();
    let line_parts_iterator = line_copy.split(";");
    let vec: Vec<&str> = line_parts_iterator.collect();

    let field0: String = vec[0].to_string();
    check_regex("^[0-9]*|^summary$".to_string(), "Invalid entry for step number in Field 0.".to_string(), &field0, &line, &line_num, &fname);

    let field1: String = vec[1].to_string();
    check_regex("^[0-9]*".to_string(), "Invalid entry for question index Field 1.".to_string(), &field1, &line, &line_num, &fname);
    
    let field2: String = vec[2].to_string();
    let f2vec: Vec<&str> = field2.split(":").collect();

    if f2vec.len() > 0 {
        if f2vec[0] == "plain" {
            check_regex("plain:NA:NA".to_string(), "Invalid entry for question type in Field 2. When question type is specified as plain, subfields 1 and 2 must be 'NA'. ".to_string(), &field2, &line, &line_num, &fname);
        }else{
            check_regex("^waitForClick".to_string(), "Invalid entry for question type in Field 2. Question type must be [ plain | waitForClick ]".to_string(), &f2vec[0].to_string(), &line, &line_num, &fname);
            if f2vec.len() > 2  && !f2vec.contains(&""){
                //println!("HERE: {}", f2vec[1]);
                check_regex("^(gameboard_?|rewardBar_?|saliencyMap_?){0,3}$".to_string(), "Invalid entry for question type in Field 2, subfield 1. Valid entries are {gameboard, rewardBar, saliencyMap}, deliminated by _.".to_string(), &f2vec[1].to_string(), &line, &line_num, &fname);
            }else{
                check_regex("$^".to_string(), "Invalid entry for question type in Field 2. Question type waitForClick cannot have blank subfields.".to_string(), &field2, &line, &line_num, &fname);
            }
        }
    }else {
        check_regex("$^".to_string(), "Invalid entry for question type in Field 2.".to_string(), &field2, &line, &line_num, &fname);
    }
    
    let mut answer_vec : Vec<String> = Vec::new();
    for x in 4..vec.len() {
        answer_vec.push(vec[x].to_string());
    }
}

fn check_regex(regex: String, msg: String, str_to_check: &String, full_line: &String, line_num: &u8, fname: &str) -> bool {
    let step_reg = Regex::new(&regex).unwrap();
    if !step_reg.is_match(&str_to_check) {
        println!("\nERROR: {}\n\t--> {}:{}\n\t|\n Line {} | {}\n\t|\n\tInvalid substring: {:?}\n", msg, fname, line_num, line_num, full_line, str_to_check);
        return true;
    }
    false
}
