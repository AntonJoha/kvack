use super::file::FileObject;
use super::position::Position;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};


pub fn init() {

    let mut pos = Position { x: 1, y: 1 };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();


    write!(stdout, "{}{}", 
           termion::clear::All, 
           termion::cursor::Goto(pos.x, pos.y ))
            .unwrap();

    stdout.flush().unwrap();


    let mut file = FileObject::new("test.txt".to_string());

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => {
                pos = file.add_char(c);
                write!(stdout, "{}{}", 
                       termion::clear::All, 
                       termion::cursor::Goto(1, 1),
                       )
                        .unwrap();
                stdout.flush().unwrap();
               
                for i in 0..file.len() {
                    write!(stdout, "{}{}\r", file.get_line(i), file.get_line(i).len()).unwrap();
                }
                write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
                stdout.flush().unwrap();
            }
            _ => ()
        }
    }
}

