
// ----- Storage Module ----- //
pub mod storage{

    //----- IMPORTS -----//
    use crate::task::Task;
    use std::fs;
    use std::io;
    



    // ----- Save Task ----- //

    // ----- tasks load the task from the Task imported.
    // ----- path accepts a string slice for the name of the task.
    // ----- the resut shows that it's either it's Ok or returns and io error.
    pub fn save_tasks(tasks: &[Task], path: &str)-> io::Result<()>{

        // ----- converts the tasks into json format
        let json = serde_json::to_string_pretty(tasks).map_err(io::Error::other)?;
        
        // ----- writes the json and to the path
        fs::write(path, json)?;

        Ok(())

    }

    // ----- Load Task ----- //
    pub fn load_task(path: &str) -> io::Result<Vec<Task>>{

        // ----- reads content of task.json into a string
        let json = fs::read_to_string(path)?;

        // ----- converst json into tasks
        let tasks: Vec<Task> = serde_json::from_str(&json).map_err(io::Error::other)?;

        // ----- returns the tasks
        Ok(tasks)
    }
}