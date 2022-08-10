use std::fs::File;
use std::io::prelude::*;

pub fn save_file(arr: &[String], arr1: &[char]) {
    let mut file = File::create("todo.txt").expect("Error creating file!");
    for i in 2_usize..arr.len() {
        if arr1[i] == '\u{2705}' {
            file.write_all(b"done! ").expect("Unable to write");
        }
        if arr1[i] == '\u{274C}' {
            file.write_all(b"not done! ").expect("Unable to write");
        }
        if arr1[i] == ' ' {
            file.write_all(b"unmarked! ").expect("Unable to write");
        }
        file.write_all((i - 1).to_string().as_bytes())
            .expect("Unable to write");
        file.write_all(b". ").expect("Unable to write");
        file.write_all(arr[i].as_bytes()).expect("Unable to write");
    }
}
