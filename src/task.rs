#[warn(unused_imports)]
use serde::{Deserialize, Serialize};
pub use task::Task;
pub use task::Priority;
pub use task::Status;

// ----- Task Module ----- //
pub mod task{
    use serde::{Deserialize, Serialize};

    // ----- Status Enum ----- //
    #[derive(Debug, Serialize, Deserialize)]
     pub enum Status{
        Pending,
        Done,
        Undone
    }

    // ----- Priority Enum ----- //
    #[derive(Debug, Serialize, Deserialize)]
    pub enum Priority{
        High,
        Moderate,
        Low
    }

    // ----- Task Struct ----- //
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Task{
        pub id: u32,
        pub title: String,
        pub priority: Priority,
        pub status: Status,
    }

    // ----- Task implementation ----- //
    impl Task{

        //------ TASK New
        pub fn new(id: u32, title: String, priority: Priority) -> Task{
            Task{
                id,
                title,
                priority,
                status: Status::Pending
            }
        }

        //------- Task Display
        pub fn display(&self) -> String{
            format!("ID: {:?} Title: {:?} Priority: {:?} Status: {:?}", 
            self.id, self.title, self.priority, self.status)
        }

        //------- Task complete
        pub fn complete(&mut self){
            self.status = Status::Done;
            println!("completed");
        }


        //------- Task Change priority
        pub fn chnage_priority(&mut self, priority: Priority){
            self.priority = priority;

        }
        

    }

}