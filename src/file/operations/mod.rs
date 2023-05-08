
use super::super::file::Files;

pub trait Operation {
    fn execute(&self, file: &mut Files);
}


pub struct Save{}

impl Operation for Save {
    fn execute(&self, file: &mut Files) {
        file.get_current_file().lock().unwrap().save();
    }
}

pub struct Close{}

impl Operation for Close {
    fn execute(&self, file: &mut Files) {
        file.close();
    }
}

