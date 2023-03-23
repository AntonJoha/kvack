use super::position::Position;


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

