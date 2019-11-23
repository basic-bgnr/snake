extern crate ncurses;

use ncurses::*;

use std::fs::File;
use std::io::Write;



use ncurses::constants::{KEY_UP, KEY_DOWN, KEY_RIGHT, KEY_LEFT, KEY_ENTER};



#[derive(PartialEq, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}


fn main() {
    

    let mut log = File::create("log.txt").unwrap();
    log.write_all(b"started\n");


    let sc = initscr();
    let (mut height, mut width) = (100, 80);
    
    getmaxyx(sc, &mut height, &mut width);
    let win = newwin(height, width, 0, 0);

    keypad(win, false);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    raw();
    noecho();
    cbreak();



    let mut snake_position: Vec<(i32, i32)> = Vec::new();
    
    snake_position.push((width/4, height/2));
    snake_position.push((width/4-1, height/2));
    snake_position.push((width/4-2, height/2));
    snake_position.push((width/4-3, height/2));
    snake_position.push((width/4-4, height/2));
    snake_position.push((width/4-5, height/2));
    snake_position.push((width/4-6, height/2));
    snake_position.push((width/4-7, height/2));
    snake_position.push((width/4-8, height/2));
    snake_position.push((width/4-9, height/2));


    let mut direction = Direction::Right; 

    loop { 
        timeout(500);


        let (head_x, head_y) = snake_position[0];
    
        log.write_all(format!("{:?}\n",direction).as_bytes());
        let key_press = getch();
        if key_press == KEY_UP {
            direction = Direction::Up;
        }
        /*direction = match key_press { 
            KEY_UP    if direction != Direction::Down  => Direction::Up, 
            KEY_DOWN  if direction != Direction::Up    => Direction::Down, 
            KEY_RIGHT if direction != Direction::Left  => Direction::Right, 
            KEY_LEFT  if direction != Direction::Right => Direction::Left,
            KEY_ENTER                                  => break,
            _                                          => direction,
        };*/
        log.write_all(format!("{:?}  {:?} ==  {:?} \n",direction, key_press, KEY_UP).as_bytes());
        log.write_all(b"\n\n");

        /*

        direction = match getch(){
            KEY_UP    => Direction::Up, 
            KEY_DOWN  => Direction::Down, 
            KEY_RIGHT => Direction::Right, 
            KEY_LEFT  => Direction::Left,
            KEY_ENTER => break,
            _         => direction,
        };
        */


        let new_head = match direction {
            Direction::Up    => (head_x, head_y - 1),
            Direction::Down  => (head_x, head_y + 1),
            Direction::Left  => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        

       let food = false;
       if food {
       }else{
           snake_position.insert(0, new_head);
           let (tail_x, tail_y) = snake_position.pop().unwrap();
           mv(tail_y, tail_x);
           addch(' ' as chtype);
       }
       for (x, y) in snake_position.iter() {
          mv(*y, *x);
          addch('#' as chtype);
       }
    }

    endwin();
    
}

fn demo() {
    initscr();

    addstr("Hello world !");

    refresh();
    getch();
    endwin();
}

fn auto_update(){
    let mut inc = 0;
    let mut direction = Direction :: Up;
    match inc {
           _ if inc%13==0 => {direction = Direction::Up;},
           _ if inc%17==0 => {direction = Direction::Left},
           
           _ if inc%7==0 => {direction = Direction::Down},
           _ if inc%11==0 => {direction = Direction::Right},
           _ => {},
       };

      inc += 1;

}
