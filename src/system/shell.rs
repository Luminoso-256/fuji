use std::collections::HashMap;
//holds the state of the shell
pub struct Shell{
    //environment variables
    environment_vars:HashMap<String, String>,//name,value
    //the current directory the shell is pointed to.
    current_directory:String,
}

impl Shell{
    //evaluate an input to this shell instance
    pub fn eval(&self, input:String){

    }
    //initialize this instance of the shell
    pub fn init(&self){

    }
}