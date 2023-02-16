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

    let mut tasks: Vec<Task> = res.unwrap().split('\n').filter(|line| line.len() != 0).map(|line| {
        let by_colon: Vec<&str> = line.split(": ").collect::<Vec<&str>>();
        let by_dash: Vec<&str> = by_colon[1].split(" - ").collect::<Vec<&str>>();
        Task { 
            topic: String::from(by_colon[0]),
            text: String::from(by_dash[0]),
            priority: by_dash[1].parse().unwrap()
        }
    }).collect();

    tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
    tasks.iter().for_each(|task| println!("{}", task));
}
