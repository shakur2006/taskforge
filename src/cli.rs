use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="taskforage")]
#[command(about="A command-line task manager")]

// ----- CLI Struct ----- //

pub struct Cli{
    #[command(subcommand)]
    pub commands: Commands,
}

// ----- Commands Enum ----- //
#[derive(Debug, Subcommand)]
pub enum Commands{
    /// Add task
    Add{
        title: String,

        #[arg(short, long, default_value="moderate")]
        priority: String,
    },

    /// List all tasks
    List,

    /// Mark a task as complete
    Complete{
        id: u32,
    },

    /// Remove task
    Remove{
        id: u32,
    },

    /// search task by title
    Search{
        query: String,
    },
}