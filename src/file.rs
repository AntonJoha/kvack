use super::position::Position;

struct Line {
    text: Vec<char>,
}


impl Line {
    fn new() -> Line {
        Line { text: Vec::new() }
    }

    fn add_char(&mut self, c: char, x: u32) {
        self.text.insert(x as usize, c);
    }
}

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

    
    pub fn add_char(&mut self, c: char) -> Position {
        match c {
            '\n' => {
                match self.lines.get_mut(self.position.y as usize) {
                    Some(line) => {
                        line.add_char(c, self.position.x as u32);
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
        let mut s = self.position.clone();
        s.x += 1;
        s.y += 1;
        s
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

}

