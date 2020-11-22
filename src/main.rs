use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let mut min_time: u32 = 30;
    let mut max_time: u32 = 300;
    let mut default_time: u32 = 180;
    let mut max_question_length: u32 = 255;
    let mut default_delete_time: u32 = 300;

    for (key, value) in env::vars() {
        match &key[..] {
            "MIN_TIME" => {min_time = value.parse().unwrap();}
            "MAX_TIME" => {max_time = value.parse().unwrap();}
            "DEFAULT_TIME" => {default_time = value.parse().unwrap();}
            "MAX_QUESTION_LENGTH" => {max_question_length = value.parse().unwrap();}
            "DEFAULT_DELETE_TIME" => {default_delete_time = value.parse().unwrap();}
            _ => {}
        }
    }
    println!("Min_Time: {}", min_time);
    println!("Max_Time: {}", max_time);
    println!("Default_Time: {}", default_time);
    println!("Max_Question_Length: {}", max_question_length);
    println!("Default_Delete_Time: {}", default_delete_time);
}
