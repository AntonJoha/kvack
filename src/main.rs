mod file;
mod position;
mod display;

use std::sync::{Arc, Mutex};
use file::FileObject;
use file::Files;
use position::Position;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};



fn main() {
    init();
}

///This is function which initilizes everything from file to position.
///It also handles the keypresses and calls the draw function.
///This is good for the moment but should be split up into a more modular design
pub fn init() {

    let mut pos = position::Position { x: 1, y: 1 };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();


    write!(stdout, "{}{}", 
           termion::clear::All, 
           termion::cursor::Goto(pos.x, pos.y ))
            .unwrap();

    stdout.flush().unwrap();

    let files: Arc<Mutex<Files>> = file::init_files();

    let file = FileObject::new("test.txt".to_string());

    match files.lock() {
        Ok(mut f) => f.push(file),
        Err(e) => println!("Error: {}", e),
    }

    for c in stdin.keys() {
        match c {
            Ok(key) => {
                match files.lock() {
                    Ok(mut f) => {
                        pos = handle_keypress(&mut f, pos, key);
                        display::draw(&mut f.get_current_file().lock().unwrap(), &mut stdout, &pos);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            },
            _ => {}
        }
        //A short wait is added to prevent it from always keeping the lock
        std::thread::sleep(std::time::Duration::from_millis(1));
        if files.lock().unwrap().len() == 0 {
            drop(stdout);
            break;
        }
    }
}

pub fn handle_keypress(files: &mut Files, pos: Position, keypress: Key) -> Position{

    let file = files.get_current_file();
    let mut f = file.lock().unwrap();

    match keypress {
        Key::Char(c) => {
            f.add_char(c)
        }
        Key::Ctrl('c') => {
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

