use super::file::FileObject;
use super::file::Files;
use super::position::Position;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};



///Draw to the terminal, one line at a time
fn draw(file: &FileObject, stdout: &mut std::io::Stdout, pos: &Position) {
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    for i in 0..file.len() {
        write!(stdout, "{}\r\n", file.get_line(i)).unwrap();
    }
    write!(stdout, "{}", termion::cursor::Goto(pos.x, pos.y)).unwrap();
    stdout.flush().unwrap();
}


pub fn handle_keypress(files: &mut Files, pos: Position, keypress: Key) -> Position{

    let mut file = files.get_current_file();
    let mut f = file.lock().unwrap();

    match keypress {
        Key::Char(c) => {
            f.add_char(c)
        }
        Key::Ctrl('c') => {
            drop(f);
            files.save_current();
            files.close_current();
            pos
        }
        Key::Up => {
            f.move_cursor(0, -1)
        },
        Key::Down => {
            f.move_cursor(0, 1)
        },
        Key::Left => {
            f.move_cursor(-1, 0)
        },
        Key::Right => {
            f.move_cursor(1, 0)
        },
        _ => pos
    }
 

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

    let mut files: Files = Files::new();

    let file = FileObject::new("test.txt".to_string());

    files.push(file);

    for c in stdin.keys() {
        match c {
            Ok(key) => {
                pos = handle_keypress(&mut files, pos, key);
                draw(&files.get_current_file().lock().unwrap(), &mut stdout, &pos);
            },
            _ => {}
        }

        if files.len() == 0 {
            drop(stdout);
            break;
        }
    }
}

