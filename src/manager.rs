use crate::task::{Task, Status};

// task manager
pub mod manager{
    use crate::Task;
    use crate::manager::Status;
    // ----- task mnager struct ----- //
    #[derive(Debug)]
    pub struct TaskManager{
        pub tasks: Vec<Task>
    }

    // ----- task manager impl block ----- //
    impl TaskManager{

        // ----- new() method ----- //
        pub fn new()-> TaskManager{
            TaskManager{
                tasks: Vec::new()
            }
        }

        // ----- add() method ----- //
        pub fn add(&mut self, task:Task) {
            self.tasks.push(task)
        }

        // ----- list() method ----- //
        pub fn list(&self) -> String{
            let mut result = String::new();

            for task in self.tasks.iter(){
                result.push_str(&format!("{}. {}\n", task.id, task.title));
            }

            result
        }

        // ----- find() method ----- //
        pub fn find(&self, id: u32)-> Option<&Task>{
            self.tasks.iter().find(|task| task.id == id)
        }

        // ----- new() method ----- //
        pub fn remove(&mut self, id:u32)-> bool{
            if let Some(task) = self.tasks.iter().find(|task| task.id == id){
                let len_before = self.tasks.len();
                self.tasks.retain(|t| t.id != id);
                if self.tasks.len() < len_before{
                    println!("contact removed successfully");
                }
               true 
            }

            else{
                println!("no task with that id exist....");
                false
            }
        }

        // -----  completed() method ----- //
        pub fn complete(&mut self, id: u32) -> bool{
            if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id){
                task.status = Status::Done;
                    true
            }
            else{
                println!("invalid if");
                false
            }
        }

    }
}