use super::file::FileObject;
use super::position::Position;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;



///Draw to the terminal, one line at a time
///At the moment it simply prints the whole file line by line. 
pub fn draw(file: &FileObject, stdout: &mut std::io::Stdout) {
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    for i in 0..file.len() {
        write!(stdout, "{}\r\n", file.get_line(i)).unwrap();
    }
    let pos = file.get_cords();
    write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
    stdout.flush().unwrap();
}

