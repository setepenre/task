#[derive(Debug)]
struct Task {
    topic: String,
    text: String,
    priority: u8,
}

impl Task {
    fn from_str(line: &str) -> std::result::Result<Task, String> {
        let by_colon: Vec<&str> = line.split(": ").collect::<Vec<&str>>();
        if by_colon.len() != 2 {
            return Err(format!("'{}' does not match format 'topic: text - priority'", line));
        }
        let by_dash: Vec<&str> = by_colon[1].split(" - ").collect::<Vec<&str>>();
        if by_dash.len() != 2 {
            return Err(format!("'{}' does not match format 'text - priority'", by_colon[1]));
        }
        match by_dash[1].parse::<u8>() {
            Ok(v) => Ok(Task { topic: String::from(by_colon[0]), text: String::from(by_dash[0]), priority: v }),
            Err(e) => Err(format!("could not parse '{}': {}", by_dash[1], e)),
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} - {}", self.topic, self.text, self.priority)
    }
}

fn usage(name: &String) -> String {
    format!("usage: {} [help] [filepath] [list|add|del] [args]\n
    help: \tprints this message and quits
    filepath: \tpath to tasks file - mandatory
    list: \tlists tasks contained in tasks file sorted by priority - default
    add: \tadd adds a task passed as a string formatted as 'topic: text - priority', priority is a 8-bit unsigned int
    del: \tdel deletes a task using its priority sorted index\n
examples:
    $ cat tasks.txt
    task: add delete feature - 1
    task: add usage - 0
    $ {} tasks.txt
    [0] task: add usage - 0
    [1] task: add delete feature - 1
    $ {} tasks.txt add task: add add feature - 2
    $ {} tasks.txt list
    [0] task: add usage - 0
    [1] task: add delete feature - 1
    [2] task: add add feature - 2
    $ {} tasks.txt del 2
    $ {} tasks.txt
    [0] task: add usage - 0
    [1] task: add delete feature - 1", name, name, name, name, name, name)
}

fn read_tasks(content: String) -> Vec<Task> {
    let lines = content.split('\n').filter(|line| line.len() != 0).map(Task::from_str);
    lines.clone().filter(|res| res.is_err()).map(|res| res.unwrap_err()).for_each(|err| eprintln!("{}", err));
    lines.filter(|res| res.is_ok()).map(|res| res.unwrap()).collect()
}

fn write_tasks(tasks: &Vec<Task>, filepath: &String) -> std::io::Result<()> {
    let content: String = tasks.iter().fold("".to_string(), |acc, task| acc + &format!("{}\n", task));
    std::fs::write(filepath, content)
}

fn list(mut tasks: Vec<Task>) {
    tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
    println!("tasks:");
    tasks.iter().enumerate().for_each(|(i, task)| println!("[{}] {}", i, task));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("missing arguments");
        eprintln!("{}", usage(&args[0]));
        std::process::exit(1);
    }

    if args[1] == "help" {
        println!("{}", usage(&args[0]));
        std::process::exit(0);
    }

    let filepath = &args[1];
    let res = std::fs::read_to_string(filepath);
    if res.is_err() {
        eprintln!("could not read {}: {}", filepath, res.unwrap_err());
        std::process::exit(1);
    }

    let mut tasks = read_tasks(res.unwrap());

    if args.len() == 2 || args[2] == "list" {
        list(tasks);
        std::process::exit(0);
    }

    if args.len() < 4 {
        eprintln!("missing arguments");
        eprintln!("{}", usage(&args[0]));
        std::process::exit(1);
    }

    if args[2] == "add" {
        let line = args[3..].join(" ");
        let res = Task::from_str(&line);
        if res.is_err() {
            eprintln!("could not read '{}' into task: {}", line, res.unwrap_err());
            std::process::exit(1);
        }
        tasks.push(res.unwrap());
        let res = write_tasks(&tasks, filepath);
        if res.is_err() {
            eprintln!("could not write tasks into {}: {}", filepath, res.unwrap_err());
            std::process::exit(1);
        }
        list(tasks);
        std::process::exit(0);
    }

    if args.len() != 4 {
        eprintln!("too many arguments");
        eprintln!("{}", usage(&args[0]));
        std::process::exit(1);
    }

    if args[2] == "del" {
        let res = args[3].parse::<u8>();
        if res.is_err() {
            eprintln!("could not parse '{}' as an index: {}", args[4], res.unwrap_err());
            std::process::exit(1);
        }
        let index = usize::from(res.unwrap());
        if !(index < tasks.len()) {
            eprintln!("'{}' is an invalid index", index);
            std::process::exit(1);
        }
        let mut indexed_tasks: Vec<(usize, &Task)> = tasks.iter().enumerate().collect();
        indexed_tasks.sort_by(|(_, a), (_, b)| a.priority.cmp(&b.priority));
        tasks.remove(indexed_tasks[index].0);
        let res = write_tasks(&tasks, filepath);
        if res.is_err() {
            eprintln!("could not write tasks into {}: {}", filepath, res.unwrap_err());
            std::process::exit(1);
        }
        list(tasks);
        std::process::exit(0);
    }

    eprintln!("unknown arguments");
    eprintln!("{}", usage(&args[0]));
    std::process::exit(1);
}
