mod save;
mod title;
mod update;
use save::save_file;
use std::io::stdin;
use std::process::Command;
use std::{thread, time};
use title::head;
use update::check_in;
use update::get_index;

fn main() {
    Command::new("clear").status().unwrap();

    let mut todo_list: Vec<String> = vec![];
    let mut todo_up: Vec<char> = vec![' ', ' '];
    let mut file_saved: bool = false;
    let mut todo_del: bool = false;
    let mut redo_stack: Vec<String> = vec![];
    let mut undo_stack: Vec<String> = vec![];

    todo_list.push(head());
    todo_list.push(" ".to_string());

    loop {
        for i in 0_usize..todo_list.len() {
            if i < 2 {
                println!("{}", todo_list[i]);
            } else {
                println!("{} {}.{}", todo_up[i], (i - 1), todo_list[i]);
            }
        }

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        if input.trim().to_string() != "" {
            if check_in(&input.trim().to_string(), "t".to_string()) {
                let i: usize = get_index(input.trim().to_string());
                todo_up.remove(i + 1);
                todo_up.insert(i + 1, '\u{2705}');
            }
            if check_in(&input.trim().to_string(), "c".to_string()) {
                let i: usize = get_index(input.trim().to_string());
                todo_up.remove(i + 1);
                todo_up.insert(i + 1, '\u{274C}');
            }
            if check_in(&input.trim().to_string(), "u".to_string()) {
                let i: usize = get_index(input.trim().to_string());
                todo_up.remove(i + 1);
                todo_up.insert(i + 1, ' ');
            }
        }

        let check_input: bool = input.trim().parse::<i32>().is_ok();
        if check_input {
            let indx = input.trim().parse::<usize>().unwrap() - 1;
            if indx <= todo_list.len() && indx <= todo_list.len() {
                let undo = todo_list[indx + 2].clone();
                undo_stack.push(undo);
                todo_del = true;
                todo_up.remove(indx + 2);
                todo_up.insert(indx + 2, ' ');
                todo_list.remove(indx + 2);
            }
        }

        if input.to_lowercase().trim() == "!u" && todo_list.len() >= 3 {
            let undo: String = todo_list[todo_list.len() - 1].clone();

            if todo_del {
                let undo = undo_stack[undo_stack.len() - 1].clone();
                todo_list.push(undo);
                todo_del = false;
            } else {
                redo_stack.push(undo);
                todo_list.pop();
            }
        }

        if input.to_lowercase().trim() == "!r" && redo_stack.len() >= 1 {
            let redo = redo_stack[redo_stack.len() - 1].clone();
            todo_list.push(redo);
            redo_stack.pop();
        }

        if input.to_lowercase().trim() == "!s" && todo_list.len() >= 3 {
            save_file(&todo_list, &todo_up);
            file_saved = true;
            println!("File saved as todo.txt!");
            thread::sleep(time::Duration::from_secs(1));
        }

        if input.to_lowercase().trim() == "!q" && todo_list.len() >= 3 {
            if file_saved {
                println!("Quiting todo-cli!")
            } else {
                println!("Quiting without saving file!")
            }
            thread::sleep(time::Duration::from_secs(1));
            break;
        }

        if input.trim().to_lowercase() == "!sq" && todo_list.len() >= 3 {
            println!("Quiting!");
            save_file(&todo_list, &todo_up);
            thread::sleep(time::Duration::from_secs(1));
            println!("File saved as 'todo.txt'!",);
            break;
        }

        if !check_input
            && input.trim().to_lowercase() != "!u"
            && input.trim().to_lowercase() != "!r"
            && input.trim() != ""
            && input.trim().to_lowercase() != "!s"
            && input.trim().to_lowercase() != "!q"
            && input.trim().to_lowercase() != "!sq"
            && !check_in(&input.trim().to_string(), "t".to_string())
            && !check_in(&input.trim().to_string(), "c".to_string())
            && !check_in(&input.trim().to_string(), "u".to_string())
        {
            todo_list.push(input);
            todo_up.push(' ');
            file_saved = false;
        }

        Command::new("clear").status().unwrap();
    }
}
