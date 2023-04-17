use super::position::Position;
use std::sync::{Arc, Mutex, mpsc::channel, mpsc::Sender, mpsc::Receiver};
use std::fs::File;
use std::io::{Write, stdout, stdin};
use std::collections::VecDeque;
use std::thread;

use operations::Operation;
mod operations;

///This is the object which holds the line data
///It is a vector of chars in order to add into any position
///Having a string causes it to crash while inserting UTF-8 at the wrong offset. 
struct Line {
    text: Vec<char>,
}


impl Line {
    fn new() -> Line {
        Line { text: Vec::new() }
    }
    
    ///Add a char to the line at the given position
    fn add_char(&mut self, c: char, x: u32) {
        self.text.insert(x as usize, c);
    }
}

///This is the object which holds the file data
///This like other objects is at the moment a bit simple and could be made better. 
pub struct FileObject {
    path: String,
    lines: Vec<Line>,
    position: Position,
}

///This is the object which holds a file
impl FileObject {
    pub fn new(path: String) -> FileObject {
        let mut lines = Vec::new();
        lines.insert(0, Line::new());
        FileObject { path: path, 
            lines: lines,
            position: Position { x: 0, y: 0 },
        }
            
    }

    ///Adds a character to the file at the current position of the cursor
    pub fn add_char(&mut self, c: char) -> Position {
        match c {
            '\n' => {
                match self.lines.get_mut(self.position.y as usize) {
                    Some(_) => {
                        self.position.x = 0;
                        self.position.y += 1;
                        self.lines.insert(self.position.y as usize, Line::new());
                    }
                    None => ()
                }
            },
            _ => {
                match self.lines.get_mut(self.position.y as usize) {
                    Some(line) => {
                        line.add_char(c, self.position.x as u32);
                        self.position.x += 1;
                    }
                    None => {
                        self.lines.insert(self.position.y as usize, Line::new());
                        self.add_char(c);
                    }
                }
            }
        }
        self.get_cords()
    }
    
    pub fn get_cords(&self) -> Position {
        Position { x: self.position.x + 1, y: self.position.y + 1 }
    }

    pub fn len(&self) -> u16 {
        self.lines.len() as u16
    }

    pub fn get_line(&self, y: u16) -> String {
        match self.lines.get(y as usize) {
            Some(line) => line.text.iter().collect(),
            None => "".to_string()
        }
    }
 

    ///Save the file to path
    pub fn save(&self) {
        let mut file = File::create(&self.path).unwrap();
        //Want to write all but the last one with a newline
        for line in &self.lines[..self.lines.len() - 1] {
            let text = line.text.iter();
            for c in text {
                file.write_all(&[*c as u8]).unwrap();
            }
            file.write_all(b"\n").unwrap();
        }
        //Write the last line without a newline
        let text = self.lines[self.lines.len() - 1].text.iter();
        for c in text {
            file.write_all(&[*c as u8]).unwrap();
        }
    }

    ///This is the function which is called to move the cursor to different positions in the file.
    ///It takes two arguments, x and y, which are the amount of characters to move the cursor in the x and y direction.
    ///It returns a Position struct which contains the new x and y coordinates of the cursor.
    ///It also guards against moving the cursor to disallowed positions.
    pub fn move_cursor(&mut self, x: i32, y: i32) -> Position {
        let mut x_temp: i32 = self.position.x as i32 + x;
        let mut y_temp: i32 = self.position.y as i32 + y;
        
        //Correct the y-axis
        while y_temp >= self.lines.len() as i32 {
            self.lines.insert(self.lines.len(), Line::new());
        }
        if y_temp < 0 {
            y_temp = 0;
        }
        self.position.y = y_temp as u16;

        //Correct the x-axis
        if x_temp > self.lines[self.position.y as usize].text.len() as i32 {
            x_temp = self.lines[self.position.y  as usize].text.len() as i32;
        }

        if x_temp < 0 {
            x_temp = 0;
        }

        self.position.x = x_temp as u16;
        
        self.get_cords()
    }

}


pub struct Files {
    files: Vec<Arc<Mutex<FileObject>>>,
    current_file: u16,
    tx: std::sync::mpsc::Sender<Box<dyn Operation + Send>>,
}

impl Files {

    pub fn new(tx: Sender<Box<dyn Operation + Send>>) -> Files {
        Files { files: Vec::new(), current_file: 0, tx}
    }

    ///Call this to get the current file that the user is editing
    pub fn get_current_file(&mut self) -> Arc<Mutex<FileObject>> {
        //If it requests a file which does not exist return a bogus one
        if self.files.len() == 0 {
            return Arc::new(Mutex::new(FileObject::new("".to_string())));
        }
        self.files[self.current_file as usize].clone()
    }
    
    ///Call this to add a new file to the list of files. 
    pub fn push(&mut self, file: FileObject) {
        self.files.push(Arc::new(Mutex::new(file)));
    }

    ///Call this to close the current file.
    ///Warning, will erase all unsaved data.
    pub fn close_current(&mut self) {
        self.tx.send(Box::new(operations::Close{})).unwrap();
    }

    pub fn close(&mut self) {
        self.files.remove(self.current_file as usize);
        if self.current_file > 0 {
            self.current_file -= 1;
        }
    }

    ///Call this to save the current file to its path.
    pub fn save_current(&mut self) {
        self.tx.send(Box::new(operations::Save{})).unwrap();
    }
    

    ///Call this to get the amount of files open at the same time. 
    pub fn len(&self) -> u16 {
        self.files.len() as u16
    }
}

pub fn init_files() -> Arc<Mutex<Files>> {
    let (tx, rx): (Sender<Box<dyn Operation + Send>>, Receiver<Box<dyn Operation + Send>>) = std::sync::mpsc::channel();
    let files = Arc::new(Mutex::new(Files::new(tx)));
    let files_clone = files.clone();
    std::thread::spawn(move || {
        loop {
            let op = rx.recv().unwrap();
            op.execute(&mut files_clone.clone().lock().unwrap());
        }
    });
    files
}

