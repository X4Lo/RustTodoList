use std::env;
use std::fmt;
use std::fs;
use std::io;

static TASKS_FILE_PATH: &str = "file path";

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

impl TaskStatus {
    fn from_str(status: &str) -> Option<TaskStatus> {
        match status.trim().to_lowercase().as_str() {
            "todo" => Some(TaskStatus::TODO),
            "doing" => Some(TaskStatus::DOING),
            "done" => Some(TaskStatus::DONE),
            _ => None,
        }
    }
}

fn read_file() -> Result<Vec<Task>, io::Error> {
    let contents = fs::read_to_string(TASKS_FILE_PATH)?;
    let mut tasks: Vec<Task> = Vec::new();

    for line in contents.split("\n") {
        let v: Vec<&str> = line.split(';').collect();

        if v.len() < 3 {
            eprintln!("Skipping malformed line: '{}'", line);
            continue;
        }

        let status = TaskStatus::from_str(v.get(2).unwrap()).unwrap();
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
    let content: String = tasks
        .iter()
        .map(|task| format!("{};{};{}\n", task.id, task.title, task.status))
        .collect::<String>()
        .strip_suffix("\n")
        .unwrap()
        .to_string();

    let _ = fs::write(TASKS_FILE_PATH, content);
}

fn add_task_menu(tasks: &mut Vec<Task>) {
    println!("----------------------------------------------");
    println!("------------------ Add Task ------------------");
    println!("----------------------------------------------");

    println!("Enter Task Title: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let title = input.trim().to_string();

    add_task(tasks, title);

    println!("Task Added!");
    println!("----------------------------------------------");
}

fn add_task(tasks: &mut Vec<Task>, title: String) {
    let id = tasks[tasks.len() - 1].id + 1;

    tasks.push(Task {
        id: id,
        title: title,
        status: TaskStatus::TODO,
    });

    save_file(tasks);
}

fn update_task_menu(tasks: &mut Vec<Task>) {
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
        eprintln!("Task ID not found.");
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
        println!("Task updated!");
    }

    println!("----------------------------------------------");
}

fn update_task(tasks: &mut Vec<Task>, id: u32, status: TaskStatus) {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.status = status;
            println!("Task updated!");
            save_file(tasks);
            return;
        }
    }
    eprintln!("Task with ID: {} not found.", id);
}

fn remove_task_menu(tasks: &mut Vec<Task>) {
    println!("----------------------------------------------");
    println!("----------------- Remove Task ----------------");
    println!("----------------------------------------------");

    println!("Enter Task ID: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let id = input.trim().parse::<u32>().unwrap();

    remove_task(tasks, id);

    println!("----------------------------------------------");
}

fn remove_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(index) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(index);
        save_file(tasks);
        println!("Task removed!")
    } else {
        eprintln!("Task with ID: {} not found.", id);
    }
}

fn view_tasks_menu(tasks: &Vec<Task>) {
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
    let args: Vec<String> = env::args().collect();
    let mut tasks = read_file().unwrap_or_else(|_| Vec::new());

    // interractive mode
    if args.len() < 2 {
        let mut input = String::new();
        println!("----------------------------------------------");
        loop {
            println!("--------- Welcome to your TODO List ----------");
            println!("----------------------------------------------");
            println!("1 - View Tasks");
            println!("2 - Add Task");
            println!("3 - Update Task");
            println!("4 - Delete Task");
            println!("0 - Quit");
            println!("# - Choice > ");

            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => view_tasks_menu(&tasks),
                "2" => add_task_menu(&mut tasks),
                "3" => update_task_menu(&mut tasks),
                "4" => remove_task_menu(&mut tasks),
                "0" => break,
                _ => eprintln!("Invalid Choice!"),
            }

            input.clear();
        }
    } else {
        // command-line mode
        match args[1].as_str() {
            "--list" | "--l" => {
                view_tasks_menu(&tasks);
            }
            "--add" | "--a" => {
                if args.len() < 3 {
                    eprintln!("Usage: --add \"task title\"");
                    return;
                }

                let title: String = args[2..].join(" ").to_string();
                add_task(&mut tasks, title);
            }
            "--update" | "--u" => {
                if args.len() < 4 {
                    eprintln!("Usage: --update <id> <status>");
                    return;
                }

                let id: u32 = args[2].parse().expect("Invalid Task ID.");

                if let Some(status) = TaskStatus::from_str(&args[3]) {
                    update_task(&mut tasks, id, status);
                } else {
                    eprintln!("Invalid status!");
                }
            }
            "--remove" | "--r" => {
                if args.len() < 3 {
                    eprintln!("Usage: --update <id> <status>");
                    return;
                }

                let id: u32 = args[2].parse().expect("Invalid Task ID.");

                remove_task(&mut tasks, id);
            }
            _ => eprintln!("Invalid arguments."),
        }
    }
}
