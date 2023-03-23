use super::file::FileObject;
use super::position::Position;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};



///Draw to the terminal, one line at a time
fn draw(file: &FileObject, stdout: &mut std::io::Stdout, pos: &Position) {
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    for i in 0..file.len() {
        write!(stdout, "{}\r\n", file.get_line(i)).unwrap();
    }
    write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
    stdout.flush().unwrap();
}

///This is function which initilizes everything from file to position.
///It also handles the keypresses and calls the draw function.
///This is good for the moment but should be split up into a more modular design
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
               
                draw(&file, &mut stdout, &pos);
            }
            Key::Ctrl('c') => break,
            Key::Up => {
                pos = file.move_cursor(0, -1);
                write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
                stdout.flush().unwrap();
            },
            Key::Down => {
                pos = file.move_cursor(0, 1);
                write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
                stdout.flush().unwrap();
            },
            Key::Left => {
                pos = file.move_cursor(-1, 0);
                write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
                stdout.flush().unwrap();
            },
            Key::Right => {
                pos = file.move_cursor(1, 0);
                write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
                stdout.flush().unwrap();
            },
            _ => ()
        }
    }
}

