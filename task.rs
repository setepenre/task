#[derive(Debug)]
struct Task {
    topic: String,
    text: String,
    priority: u8,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} - {}", self.topic, self.text, self.priority)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} [filepath]", args[0]);
        std::process::exit(1);
    }
    let filepath = &args[1];
    let res = std::fs::read_to_string(filepath);
    if res.is_err() {
        eprintln!("could not read {}: {}", filepath, res.unwrap_err());
        std::process::exit(1);
    }

    let content = res.unwrap();
    let lines = content.split('\n').filter(|line| line.len() != 0).map(|line| {
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
    });

    lines.clone().filter(|res| res.is_err()).map(|res| res.unwrap_err()).for_each(|err| eprintln!("{}", err));

    let mut tasks: Vec<Task> = lines.filter(|res| res.is_ok()).map(|res| res.unwrap()).collect();
    tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
    println!("tasks:");
    tasks.iter().enumerate().for_each(|(i, task)| println!("[{}] {}", i, task));
}
