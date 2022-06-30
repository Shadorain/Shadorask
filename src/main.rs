use std::collections::HashMap;
use serde_repr::*;
use clap::Parser;

const DB_FILENAME: &str = ".todos.json";


/* Argument Data */

#[derive(Parser)]
#[clap(name = "Shadotask", version, about = "A tool to manage tasks from the shadows")]
struct Cli {
    #[clap(subcommand)]
    sub_cmd: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Add {
        #[clap(value_parser)]
        todo: Vec<String>,
    },
    #[clap(arg_required_else_help = true)]
    Complete {
        #[clap(required = true, value_parser)]
        todo: String,
    },
}


/* Todo Data {{{ */

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum TodoState {
    Active = 0, InProgress, Holding, Complete
}

struct Todo {
    map: HashMap<String, TodoState>,
}

impl Todo {
    fn insert(&mut self, list: &Vec<String>) {
        for t in list {
            self.map.insert(t.to_string(), TodoState::Active);
        }
    }

    fn _insert_one(&mut self, key: String) {
        self.map.insert(key, TodoState::Active);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new().write(true)
            .create(true).open(DB_FILENAME)?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new().write(true)
            .create(true).read(true).open(DB_FILENAME)?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = TodoState::Complete),
            None => None,
        }
    }
}
/* }}} */

fn main() {
    let args = Cli::parse();
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{:?}, {:?}", action, item);

    let mut todo_ctx = Todo::new().expect("Initialization of db failed");

    match &args.sub_cmd {
        Commands::Add { todo } => {
            todo_ctx.insert(todo);
            match todo_ctx.save() {
                Ok(_) => println!("todo saved"),
                Err(e) => println!("An error occured: {}", e),
            }
        }
        Commands::Complete { todo } => {
            match todo_ctx.complete(todo) {
                None => println!("'{}' is not present in the list", todo),
                Some(_) => match todo_ctx.save() {
                    Ok(_) => println!("todo saved"),
                    Err(e) => println!("An error occured: {}", e),
                }
            }
        }
    }
}
