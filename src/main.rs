extern crate ncurses;
extern crate rand;

use ncurses::*;

use rand::Rng;

use Constants::{KeyDown, KeyEsc, KeyLeft, KeyRight, KeyUp};

enum Constants {
    KeyUp = 'k' as isize,
    KeyDown = 'j' as isize,
    KeyLeft = 'h' as isize,
    KeyRight = 'l' as isize,
    KeyEsc = 27 as isize,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn main() {
    let mut rng = rand::thread_rng();

    let sc = initscr();
    let (mut height, mut width) = (100, 80);

    getmaxyx(sc, &mut height, &mut width);
    let win = newwin(height, width, 0, 0);

    keypad(win, true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    noecho();
    cbreak();

    let mut snake_position: Vec<(i32, i32)> = Vec::new();
    let length = 20_i32;
    (0..=length).map(|x| snake_position.push((width/4 - x, height/2)))
                .last();


    let mut direction = Direction::Right;
    let (mut food_x, mut food_y) = (width / 2, height / 2);
    let mut score = 0;

    loop {
        timeout(100);


        border('|' as chtype, 
               '|' as chtype, 
               '-' as chtype, 
               '-' as chtype, 
               '+' as chtype, 
               '+' as chtype, 
               '+' as chtype, 
               '+' as chtype);

        direction = match getch() {
            s if (KeyUp as i32)    == s && direction != Direction::Down  => Direction::Up,
            d if (KeyDown as i32)  == d && direction != Direction::Up    => Direction::Down,
            f if (KeyRight as i32) == f && direction != Direction::Left  => Direction::Right,
            a if (KeyLeft as i32)  == a && direction != Direction::Right => Direction::Left,
            esc if (KeyEsc as i32) == esc                                => break,
            _                                                            => direction,
        };

        let (head_x, head_y) = snake_position[0];

        let new_head = match direction {
            Direction::Up    => (head_x, head_y - 1),
            Direction::Down  => (head_x, head_y + 1),
            Direction::Left  => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        if new_head.0 <= 0 || new_head.0 >= width {
            break;
        }
        if new_head.1 <= 0 || new_head.1 >= height {
            break;
        }
       
        mvaddch(food_y, food_x, '#' as chtype);

        if new_head == (food_x, food_y) {
            food_x = rng.gen_range(1, width - 1);
            food_y = rng.gen_range(1, height - 1);
            score += 1;
        } else {
            let (tail_x, tail_y) = snake_position.pop()
                                                 .unwrap();
            mvaddch(tail_y, tail_x, ' ' as chtype);
        }
        snake_position.insert(0, new_head);

        snake_position.iter()
                      .map(|(x, y)| mvaddch(*y, *x, '*' as chtype))
                      .last();
        

        doupdate();
        refresh();
        getmaxyx(sc, &mut height, &mut width);
    }
    
    
    let exit_message_1 = format!("Your score is : {:?}\0", score as u8).as_bytes()
                                                                       .iter()
                                                                       .map(|&x| x as chtype)
                                                                       .collect::<Vec<chtype>>();
    
    let exit_message_2 = "Press any key to exit....\0".as_bytes()
                                                       .iter()
                                                       .map(|&x| x as chtype)
                                                       .collect::<Vec<chtype>>();
                                     
    mvaddchstr(height/2, 
               width/2 - (exit_message_1.len() as i32)/2, 
               &exit_message_1);

    mvaddchstr(height-2, 
               2, 
               &exit_message_2);
   
    
    doupdate();
    halfdelay(255);
    
    getch();   
    echo();
    endwin();
}
