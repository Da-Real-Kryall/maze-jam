//const SIZE: (usize, usize) = (200,100); // (x, y)
const START_POS: (usize, usize) = (0, 0); // (x, y)
const STEP: usize = 50;
const DELAY_MS: u64 = 0;

fn draw_grid(old_grid: Vec<Vec<i32>>, grid: Vec<Vec<i32>>, mode: i32) {
    //goto 0,0
    let mut print_buffer = String::new();
    let mut jump = false;
    let mut currently_flashing: bool = false;

    print_buffer += format!("\x1B[{};{}H", 1, 1).as_str();

    for y in 0..grid.len()/2 {
        for x in 0..grid[0].len() {
            let mut halve_colours: [usize; 2] = [0,0]; //top, bottom
            let mut old_halve_colours: [usize; 2] = [0,0]; //top, bottom
            for dy in 0..=1 {
                
                for i in 0..=1 {
                    let col;
                    if i == 0 {
                        col = grid[2*y+dy][x];
                    } else {
                        col = old_grid[2*y+dy][x];
                    }
                    let chr = match mode { //palette: "█#. " -> 3210
                        1..=2 => {//floodfilling and removing dead ends
                            match col {
                                1 => 3,
                                2..=3 => 2,
                                _ => 0,
                            }
                        },
                        3..=4 => {//animating the ant (A) moving from start to end
                            match col {
                                1 => 3,
                                2 => 1,
                                4..=5 => 2,
                                _ => 0
                            }
                        },
                        _ => {//making the maze (0)
                            match col {
                                0..=1 => 3,
                                4 => 0,
                                _ => 2
                            }
                        },
                    };
                    if i == 0 {
                        halve_colours[dy] = chr;
                    } else {
                        old_halve_colours[dy] = chr;
                    }
                }
            };
            //256 colour mode colours for black, dark grey, light grey, white
            let palette = [16, 240, 249, 231];

            if halve_colours != old_halve_colours {
                print_buffer += format!("\x1B[{};{}H\x1B[0m", y+1, x+1).as_str();
                //make it flash if mode is 4 and the cell is light grey
                //if mode == 4 && halve_colours[1] == 2 {
                //    if currently_flashing == false {
                //        print_buffer += "\x1B[5m";
                //        currently_flashing = true;
                //    } else {
                //        print_buffer += "\x1B[0m";
                //        currently_flashing = false;
                //    }
                //}
                //set foreground colour to the top half colour
                print_buffer += format!("\x1B[38;5;{}m", palette[halve_colours[0]]).as_str();
                //set background colour to the bottom half colour
                print_buffer += format!("\x1B[48;5;{}m", palette[halve_colours[1]]).as_str();
                //print a top half block character
                print_buffer += format!("▀").as_str();
            }
        }
    }
    println!("{}", print_buffer);
}


//ms delay
use std::thread;

use rand::{thread_rng, seq::SliceRandom};

//termion for screen size
use termion::terminal_size;



fn main() {
    //clear the screen
    print!("\x1B[2J");
    //hide cursor
    print!("\x1B[?25l");



    //on the grid, 0 is unexplored, 1 is a wall, 2 is explored
    /*
    0 1 2 3 4 5 6
    1   #   #   #
    2 # # # # # #
    3   #   #   #
    4 # # # # # #
    5   #   #   #
    6 # # # # # #
     */

    //terminal size
    let SIZE: (usize, usize) = ((terminal_size().unwrap().0 as usize-2)/2, (((terminal_size().unwrap().1*2)/2) as usize-2));
    let mut reset_grid: Vec<Vec<i32>> = vec![vec![10; SIZE.0*2+1]; SIZE.1*2+1];
    let mut grid: Vec<Vec<i32>> = vec![vec![1; SIZE.0*2+1]; SIZE.1*2+1];
    for y in 0..SIZE.1 {
        for x in 0..SIZE.0 {
            grid[y*2+1][x*2+1] = 0;
        }
    }
    grid[START_POS.1+1][START_POS.0+1] = 2;

    //at the very right of the screen, print a full block character on every line
    for y in 0..SIZE.1 {
        print!("\x1B[{};{}H", y+1, SIZE.0*2+1);
        print!("█");
    }

    //at the very bottom of the screen, print an upper half block character
    //first goto the bottom
    println!("\x1B[{};{}H", SIZE.1, 1);
    //set colour to white
    print!("\x1B[38;5;231m");
    //then print the characters all along
    for x in 0..SIZE.0*2 {
        print!("▀");
    }
    print!("▀");


    let mut ant_pos: (usize, usize) = (START_POS.0+1, START_POS.1+1);
    let mut depth: i32 = 1;
    let mut backtracking = false;
    let mut done_with_generation = false;
    let mut max_depth_reached: i32 = 0;
    let mut max_depth_reached_pos: (usize, usize) = START_POS;
    let mut i = 0;
    let mut old_grid = grid.clone();
    draw_grid(reset_grid.clone(), grid.clone(), 0);
    while done_with_generation == false {
        
        i += 1;
        let mut attempted_directions: Vec<i32> = vec![0, 1, 2, 3];
        attempted_directions.shuffle(&mut thread_rng());
        

        let mut dead_end = true;
        
        for d in attempted_directions.iter() {
            //(0 is +x, 1 is +y, 2 is -x, 3 is -y)
            match d {
                0 => {
                    //check if there is a wall to the right and if the tile beyond that is explored / in the grid
                    if ant_pos.0+2 < SIZE.0*2+1 && grid[ant_pos.1][ant_pos.0+1] == 1 && grid[ant_pos.1][ant_pos.0+2] == 0 {
                        if backtracking == false {
                            grid[ant_pos.1][ant_pos.0+1] = 2;
                            ant_pos.0 += 2;
                        }
                        dead_end = false;
                        break;
                    }
                },
                1 => {
                    if ant_pos.1+2 < SIZE.1*2+1 && grid[ant_pos.1+1][ant_pos.0] == 1 && grid[ant_pos.1+2][ant_pos.0] == 0 {
                        if backtracking == false {
                            grid[ant_pos.1+1][ant_pos.0] = 2;
                            ant_pos.1 += 2;
                        }
                        dead_end = false;
                        break;
                    }
                },
                2 => {
                    if ant_pos.0 >= 2 && grid[ant_pos.1][ant_pos.0-1] == 1 && grid[ant_pos.1][ant_pos.0-2] == 0 {
                        if backtracking == false {
                            grid[ant_pos.1][ant_pos.0-1] = 2;
                            ant_pos.0 -= 2;
                        }
                        dead_end = false;
                        break;
                    }
                },
                3 => {
                    if ant_pos.1 >= 2 && grid[ant_pos.1-1][ant_pos.0] == 1 && grid[ant_pos.1-2][ant_pos.0] == 0 {
                        if backtracking == false {
                            grid[ant_pos.1-1][ant_pos.0] = 2;
                            ant_pos.1 -= 2;
                        }
                        dead_end = false;
                        break;
                    }
                },
                _ => println!("Error"),
            }    
        }
        if dead_end == false {
            backtracking = false;
        } else {
            backtracking = true;
        }
        if backtracking == false && dead_end == false {
            depth += 1;
        }
        if backtracking == false {
            grid[ant_pos.1][ant_pos.0] = -depth;
        } else {
            grid[ant_pos.1][ant_pos.0] = 4;
        }
        if backtracking {
            //pick the neighbouring cell 2 steps in each cardinal direction with the lowest value
            let mut lowest_value = 0;
            let mut lowest_value_direction = 0;
            for d in 0..4 {
                let mut temp_value = 0;
                match d {
                    0 => {
                        if ant_pos.0+2 < SIZE.0*2+1 && grid[ant_pos.1][ant_pos.0+1] == 2 {
                            temp_value = grid[ant_pos.1][ant_pos.0+2];
                        }
                    },
                    1 => {
                        if ant_pos.1+2 < SIZE.1*2+1 && grid[ant_pos.1+1][ant_pos.0] == 2 {
                            temp_value = grid[ant_pos.1+2][ant_pos.0];
                        }
                    },
                    2 => {
                        if ant_pos.0 >= 2 && grid[ant_pos.1][ant_pos.0-1] == 2 {
                            temp_value = grid[ant_pos.1][ant_pos.0-2];
                        }
                    },
                    3 => {
                        if ant_pos.1 >= 2 && grid[ant_pos.1-1][ant_pos.0] == 2 {
                            temp_value = grid[ant_pos.1-2][ant_pos.0];
                        }
                    },
                    _ => println!("Error"),
                }
                
                if temp_value < lowest_value && temp_value < 0 {
                    lowest_value = temp_value;
                    lowest_value_direction = d;
                }
                if lowest_value == 0 {
                    lowest_value_direction = 4;
                }
            }
            //move the ant in the direction of the lowest value and set the cell it was on to 4 (has been identified as a dead end)
            match lowest_value_direction {
                0 => {
                    grid[ant_pos.1][ant_pos.0+1] = 4;
                    ant_pos.0 += 2;
                },
                1 => {
                    grid[ant_pos.1+1][ant_pos.0] = 4;
                    ant_pos.1 += 2;
                },
                2 => {
                    grid[ant_pos.1][ant_pos.0-1] = 4;
                    ant_pos.0 -= 2;
                },
                3 => {
                    grid[ant_pos.1-1][ant_pos.0] = 4;
                    ant_pos.1 -= 2;
                },
                4 => {
                    done_with_generation = true;
                },
                _ => println!("Error"),
            }
            depth -= 1;
        }
        //if depth > max_depth_reached {
        //    max_depth_reached = depth;
        //    max_depth_reached_pos = ant_pos;
        //}
        if i % STEP == 0 {
            draw_grid(old_grid, grid.clone(), 0);
            old_grid = grid.clone();
            //println!("Creating maze, current depth: {}                                           ", depth);
            thread::sleep(std::time::Duration::from_millis(DELAY_MS));
        }
        //println!("Creating maze, current depth: {}                                           ", depth);
        //thread::sleep(std::time::Duration::from_millis(10));
    }
    //set max depth reached pos to the bottom right corner of maze
    max_depth_reached_pos = (SIZE.0*2-1, SIZE.1*2-1);


    //thread::sleep(std::time::Duration::from_millis(2000));

    //println!("Max depth reached: {} at {:?}, floodfilling to find optimum path", max_depth_reached, max_depth_reached_pos);
    
    /*
    let amogus: [[i32; 4]; 4] = [
        [0, 0, 0, 1],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
        [0, 1, 0, 1]
    ];

    for x in 0..4 {
        for y in 0..4 {
            grid[y+SIZE.1][x+SIZE.0] = amogus[y as usize][x as usize];
        }
    }
    */

    for y in 0..SIZE.1*2+1 {
        for x in 0..SIZE.0*2+1 {
            grid[y][x] = match grid[y][x] {
                0 => 0,
                1 => 1,
                2 => 0,
                3 => 0,
                4 => 0,
                _ => 0,
            }
        }
    }

    grid[START_POS.1+1][START_POS.0+1] = 2;

    let mut new_grid = grid.clone();
    let mut old_draw_grid = grid.clone();
    //draw_grid(reset_grid.clone(), grid.clone(), 1);
    //now loop over the whole thing with a floodfill from the start until the end pos gets filled
    while grid[max_depth_reached_pos.1][max_depth_reached_pos.0] != 2 {//the end will turn from 3 to 4, normal filled cells will go from 0 to 2
        i += 1;
        for y in 1..SIZE.1*2 {
            for x in 1..SIZE.0*2 {
                if grid[y][x] == 2 {
                    if grid[y-1][x] == 0 {
                        new_grid[y-1][x] = 2;
                    }
                    if grid[y+1][x] == 0 {
                        new_grid[y+1][x] = 2;
                    }
                    if grid[y][x-1] == 0 {
                        new_grid[y][x-1] = 2;
                    }
                    if grid[y][x+1] == 0 {
                        new_grid[y][x+1] = 2;
                    }
                }
            }
        }
        if i % STEP == 0 {    
            draw_grid(old_draw_grid.clone(), new_grid.clone(), 1);
            old_draw_grid = grid.clone();
            thread::sleep(std::time::Duration::from_millis(DELAY_MS));
        }
        grid = new_grid.clone();
        //println!("Retracting dead ends to isolate correct path...             ");
    }
    grid[max_depth_reached_pos.1][max_depth_reached_pos.0] = 3;
    grid[START_POS.1+1][START_POS.0+1] = 3;

    //now loop over again, this time setting any cells with one cardinal neighbours set to 2, to 0
    new_grid = grid.clone();
    old_draw_grid = grid.clone();
    loop {
        i += 1;
        for y in 0..SIZE.1*2+1 {
            for x in 0..SIZE.0*2+1 {
                if grid[y][x] == 2 && (
                    (y > 0 && grid[y-1][x] >= 2) as i32 +
                    (y < SIZE.1*2 && grid[y+1][x] >= 2) as i32 +
                    (x > 0 && grid[y][x-1] >= 2) as i32 +
                    (x < SIZE.0*2 && grid[y][x+1] >= 2) as i32 == 1
                ) {
                    new_grid[y][x] = 0;
                }
            }
        }
        if grid == new_grid {
            break;
        };
        if i%(STEP/5+1) == 0 {
            draw_grid(old_draw_grid.clone(), new_grid.clone(), 2);
            old_draw_grid = grid.clone();
            thread::sleep(std::time::Duration::from_millis(DELAY_MS));
        }
        grid = new_grid.clone();
    }

    grid[max_depth_reached_pos.1][max_depth_reached_pos.0] = 2;

    //thread::sleep(std::time::Duration::from_millis(500));
    draw_grid(reset_grid.clone(), grid.clone(), 3);

    //now, animate an ant (represented by a cell set to 4) moving from the start to the end
    let mut ant_pos: (usize, usize) = (START_POS.0+1, START_POS.1+1);
    //the algorithm will consist of the ant moving to an adjacent cell that is set to 2, setting the initial cell to 0, and repeating until the end is reached

    old_draw_grid = grid.clone();
    while ant_pos != max_depth_reached_pos {
        i += 1;
        let prev_ant_pos = ant_pos;
        if grid[ant_pos.1][ant_pos.0+1] == 2 {
            ant_pos.0 += 1;
        } else if grid[ant_pos.1+1][ant_pos.0] == 2 {
            ant_pos.1 += 1;
        } else if grid[ant_pos.1][ant_pos.0-1] == 2 {
            ant_pos.0 -= 1;
        } else if grid[ant_pos.1-1][ant_pos.0] == 2 {
            ant_pos.1 -= 1;
        }
        grid[prev_ant_pos.1][prev_ant_pos.0] = 5;
        grid[ant_pos.1][ant_pos.0] = 4;
        if i%(STEP/5+1) == (STEP/5+1)-1{
            draw_grid(old_draw_grid.clone(), grid.clone(), 3);
            old_draw_grid = grid.clone();
            thread::sleep(std::time::Duration::from_millis(DELAY_MS));
        }
        
    }
    //move to bottom of screen and print an ln
    draw_grid(reset_grid.clone(), grid.clone(), 4);
    print!("\x1B[{};{}H", SIZE.1*2+2, 1);
    
}
