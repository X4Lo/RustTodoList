use std::fmt;
use std::fs;
use std::io;
use std::process::exit;

static TASKS_FILE_PATH: &str =
    "file path";

struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

#[derive(Debug)]
enum TaskStatus {
    TODO,
    DOING,
    DONE,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            TaskStatus::TODO => "TODO",
            TaskStatus::DOING => "DOING",
            TaskStatus::DONE => "DONE",
        };
        write!(f, "{}", status_str)
    }
}

fn read_file() -> Result<Vec<Task>, io::Error> {
    let contents = fs::read_to_string(TASKS_FILE_PATH)?;
    let mut tasks: Vec<Task> = Vec::new();

    for line in contents.split("\n") {
        let v: Vec<&str> = line.split(';').collect();

        if v.len() < 3 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        println!("Raw status: {:?}", v[2]); // Debug output

        let status = match v[2].trim() {
            TODO => TaskStatus::TODO,
            DOING => TaskStatus::DOING,
            DONE => TaskStatus::DONE,
            _ => {
                eprintln!("Invalid status: '{}'", v[2]);
                panic!("Invalid status");
            }
        };

        let task = Task {
            id: v.get(0).unwrap().parse::<u32>().unwrap(),
            title: v.get(1).unwrap().to_string(),
            status: status,
        };

        tasks.push(task);
    }

    Ok(tasks)
}

fn save_file(tasks: &Vec<Task>) {
    let mut content: String = String::new();

    for task in tasks.iter() {
        content.push_str(&format!("{};{};{}\n", task.id, task.title, task.status));
    }

    fs::write(TASKS_FILE_PATH, content);
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("----------------------------------------------");
    println!("------------------ Add Task ------------------");
    println!("----------------------------------------------");

    println!("Enter Task Title: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let id = tasks[tasks.len() - 1].id + 1;
    let title = input.trim().to_string();

    tasks.push(Task {
        id: id,
        title: title,
        status: TaskStatus::TODO,
    });

    save_file(tasks);
    println!(">>>>> Task Added! <<<<<");
    println!("----------------------------------------------");
}

fn update_task(tasks: &mut Vec<Task>) {
    println!("----------------------------------------------");
    println!("----------------- Update Task ----------------");
    println!("----------------------------------------------");

    println!("Enter Task ID: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let id = input.trim().parse::<u32>().unwrap();

    let mut index: Option<usize> = None;
    for (i, task) in tasks.iter().enumerate() {
        if task.id == id {
            index = Some(i);
            break;
        }
    }

    if index.is_none() {
        println!(">>>>> Task ID Not Found! <<<<<");
    } else {
        println!("* - Available Status:");
        println!("1 - TODO");
        println!("2 - DOING");
        println!("3 - DONE");
        println!("Choose a new status: ");

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let status_number = input.trim().parse::<u32>().unwrap();

        let status = match status_number {
            1 => TaskStatus::TODO,
            2 => TaskStatus::DOING,
            3 => TaskStatus::DONE,
            _ => {
                eprintln!("Invalid status: '{}'", status_number);
                panic!("Invalid status");
            }
        };

        tasks[index.unwrap()].status = status;

        save_file(tasks);
        println!(">>>>> Task Updated! <<<<<");
    }

    println!("----------------------------------------------");
}

fn remove_task(tasks: &mut Vec<Task>) {
    println!("----------------------------------------------");
    println!("----------------- Remove Task ----------------");
    println!("----------------------------------------------");

    println!("Enter Task ID: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let id = input.trim().parse::<u32>().unwrap();

    let mut index: Option<usize> = None;
    for (i, task) in tasks.iter().enumerate() {
        if task.id == id {
            index = Some(i);
            break;
        }
    }

    if index.is_none() {
        println!(">>>>> Task ID Not Found! <<<<<");
    } else {
        tasks.remove(index.unwrap());
        save_file(tasks);
        println!(">>>>> Task Removed! <<<<<");
    }

    println!("----------------------------------------------");
}

fn view_tasks(tasks: &Vec<Task>) {
    println!("----------------------------------------------");
    println!("-------------- Your Tasks Are ----------------");
    println!("----------------------------------------------");

    if tasks.len() == 0 {
        println!("No Task Found.")
    } else {
        for task in tasks.iter() {
            println!("{} - {} ({:?})", task.id, task.title, task.status)
        }
    }
    println!("----------------------------------------------");
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut input = String::new();
    let mut choice: u32 = 99;

    match read_file() {
        Ok(t) => tasks = t,
        Err(e) => println!("Error reading file: {}", e),
    }

    println!("----------------------------------------------");
    while choice != 0 {
        println!("--------- Welcome to your TODO List ----------");
        println!("----------------------------------------------");
        println!("1 - View Tasks");
        println!("2 - Add Task");
        println!("3 - Update Task");
        println!("4 - Delete Task");
        println!("0 - Quit");
        println!("# - Choice > ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice = input.trim().parse();

        match choice {
            Ok(num) => println!("You entered: {}", num),
            Err(_) => eprintln!("Invalid input! Please enter a valid number."),
        }

        input.clear();

        match choice.unwrap() {
            1 => view_tasks(&tasks),
            2 => add_task(&mut tasks),
            3 => update_task(&mut tasks),
            4 => remove_task(&mut tasks),
            0 => exit(0),
            _ => println!("Invalid Choice!"),
        }
    }
}
