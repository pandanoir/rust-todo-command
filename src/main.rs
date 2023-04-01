/*
   コマンド概要
   $ todo
     TODO 一覧を表示する
   $ todo add task
     task を追加する
   $ todo check task
     task を完了させる
   $ todo delete task
     task を削除する
*/
use std::{fmt::Display, io::{BufRead, BufReader, Write}};
use std::{env, fs::File, process};

#[derive(Clone)]
struct Todo {
    title: String,
    checked: bool,
}
impl Todo {
    pub fn new(title: impl Into<String>, checked: bool) -> Todo {
        Todo {
            title: title.into(),
            checked,
        }
    }
}
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}",
            if self.checked { "x" } else { " " },
            self.title.as_str()
        )
    }
}

fn show_todos(todos: &Vec<Todo>) {
    for todo in todos {
        println!("{}", todo)
    }
}

fn check(todos: &Vec<Todo>, task: &str) -> Vec<Todo> {
    match todos.iter().position(|todo| todo.title == task) {
        Some(index) => {
            let mut new_todos = todos.clone();
            new_todos[index].checked = true;
            new_todos.to_vec()
        }
        None => todos.clone(),
    }
}

fn write_todos_to_file(todos: &Vec<Todo>) -> Result<(), std::io::Error> {
    match File::create("todo.txt") {
        Ok(mut file) => write!(
            file,
            "{}",
            todos
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        ),
        Err(err) => Err(err),
    }
}

fn main() {
    let todos = match File::open("todo.txt") {
        Err(_) => vec![],
        Ok(file) => BufReader::new(file)
            .lines()
            .filter_map(|line| {
                if let Ok(line) = line {
                    Some(Todo::new(&line[4..], line.starts_with("[x]")))
                } else {
                    None
                }
            })
            .collect(),
    };

    if env::args().len() == 1 {
        show_todos(&todos);
        process::exit(1);
    }

    let command = env::args().nth(1).unwrap();

    if command == "check" {
        match env::args().nth(2) {
            None => println!("check command must take one argument"),
            Some(task) => {
                let todos = check(&todos, &task);
                show_todos(&todos);
                write_todos_to_file(&todos).unwrap()
            }
        }
        process::exit(1);
    }
    if command == "add" {
        match env::args().nth(2) {
            None => println!("add command must take one argument"),
            Some(task) => {
                let todos = [todos, vec![Todo::new(task, false)]].concat();
                show_todos(&todos);
                write_todos_to_file(&todos).unwrap()
            }
        }
        process::exit(1);
    }
    if command == "delete" {
        match env::args().nth(2) {
            None => println!("delete command must take one argument"),
            Some(task) => {
                let todos = todos.into_iter().filter(|x| x.title != task).collect();
                show_todos(&todos);
                write_todos_to_file(&todos).unwrap()
            }
        }
        process::exit(1);
    }
}
