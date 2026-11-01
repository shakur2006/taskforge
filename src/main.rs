// ----- Crate Imports ----- //
mod task;
mod manager;
mod storage;
mod cli;
mod error;

use clap::Parser;
use cli::{Commands,Cli};
use error::TaskError;
use manager::manager::TaskManager;
use std::fs;
use std::path::Path;
use task::{Priority, Task};
use storage::storage::{save_tasks, load_task};
// ----- Constant ----- //
const DATA_FILE: &str = "data/task.json";

// ----- Mian Functiion ----- //
fn main() -> Result<(), TaskError>{
    let cli = Cli::parse();
    // ----- ensue data file directory exist
    fs::create_dir_all("data")?;

    // ----- load existinig task or start with empty list
    let task = if Path::new(DATA_FILE).exists(){
        load_task(DATA_FILE).unwrap_or_else(|_| Vec::new())
    } else{
        Vec::new()
    };


    let mut manager = TaskManager::new();
    manager.tasks = task;

    match cli.commands{
        
        
        Commands::Add {title, priority } => {
            let priority = match priority.to_lowercase().as_str() {
                "high" => Priority::High,
                "moderate" => Priority::Moderate,
                "low" => Priority::Low,
                _ => {
                    return Err(TaskError::InvalidInput(
                        "Priority must be high, moderate, or low.".to_string(),
                    ));
                }
            };

            let id = manager
                .tasks
                .iter()
                .map(|task| task.id)
                .max()
                .unwrap_or(0)
                .checked_add(1)
                .ok_or_else(|| {
                    TaskError::InvalidInput(
                        "No more task IDs are available.".to_string(),
                    )
                })?;

            let task = Task::new(id, title, priority);
            manager.add(task);

            save_tasks(&manager.tasks, DATA_FILE)?;
            println!("Task added successfully.");
        }

        Commands::List => {
            if manager.tasks.is_empty() {
                println!("No tasks found.");
            } else {
                print!("{}", manager.list());
            }
        }

        
        Commands::List => {
            if manager.tasks.is_empty(){
                println!("no task found");
            }
            else{
                print!("{}", manager.list());
            }
        }


        Commands::Complete { id } => {
            if manager.complete(id){
                save_tasks(&manager.tasks, DATA_FILE)?;
                println!("Task with {id} completed ");
            } else{
                return Err(TaskError::TaskNotFound(id));
            }
        }


        Commands::Remove { id } => {

            if manager.remove(id){
                save_tasks(&manager.tasks, DATA_FILE)?;
                println!("task with  {id} removed successfuly");
            } else{
                return Err(TaskError::TaskNotFound(id));
            }
        }


        Commands::Search { query } => {
            let matches: Vec<_> = manager
                .tasks
                .iter()
                .filter(|task|{
                    task.title
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                })
                .collect();

            if matches.is_empty(){
                println!("No mathcinig tasks found");
            } else{
                for task in matches{
                    println!("{}", task.display());
                }
            }
        }


    }


    Ok(())
}
