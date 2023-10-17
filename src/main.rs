const SIZE: (usize, usize) = (30, 15); // (x, y)
const START_POS: (usize, usize) = (0, 0); // (x, y)

fn draw_grid(grid: [[i32; SIZE.0*2+1]; SIZE.1*2+1], mode: i32) {
    //goto 0,0
    print!("\x1B[{};{}H", 0, 0);
    for row in grid.iter() {
        for col in row.iter() {
            match mode {
                0 => {//making the maze
                    match col {
                        0 => print!("█"),
                        1 => print!("█"),
                        2 => print!("."),
                        3 => print!("#"),
                        4 => print!(" "),
                        _ => print!(" "),
                    }
                },
                1 => {//floodfilling
                    match col {
                        0 => print!(" "),
                        1 => print!("█"),
                        2 => print!("#"),
                        3 => print!("."),
                        4 => print!(" "),
                        _ => print!(" "),
                    }
                },
                2 => {//removing dead ends
                    match col {
                        0 => print!(" "),
                        1 => print!("█"),
                        2 => print!("#"),
                        3 => print!("."),
                        4 => print!(" "),
                        _ => print!(" "),
                    }
                },
                3 => {//animating the ant (A) moving from start to end
                    match col {
                        0 => print!(" "),
                        1 => print!("█"),
                        2 => print!("."),
                        3 => print!(" "),
                        4 => print!("A"),
                        _ => print!(" "),
                    }
                },
                _ => println!("Error"),
            }
        }
        println!();
    }
}


//ms delay
use std::thread;

use rand::{thread_rng, seq::SliceRandom};





fn main() {
    //clear the screen
    print!("\x1B[2J");
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

    let mut grid: [[i32; SIZE.0*2+1]; SIZE.1*2+1] = [[1; SIZE.0*2+1]; SIZE.1*2+1];
    for y in 0..SIZE.1 {
        for x in 0..SIZE.0 {
            grid[y*2+1][x*2+1] = 0;
        }
    }


    let mut ant_pos: (usize, usize) = (START_POS.0+1, START_POS.1+1);
    let mut depth: i32 = 1;
    let mut backtracking = false;
    let mut done_with_generation = false;
    let mut max_depth_reached: i32 = 0;
    let mut max_depth_reached_pos: (usize, usize) = START_POS;
    
    while done_with_generation == false {
        let mut attempted_directions: Vec<i32> = vec![0, 1, 2, 3];
        attempted_directions.shuffle(&mut thread_rng());
        
        grid[ant_pos.1][ant_pos.0] = -depth;
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
            if backtracking == false {
                depth += 1;
            }
            backtracking = false;
        } else {
            backtracking = true;
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
        if depth > max_depth_reached {
            max_depth_reached = depth;
            max_depth_reached_pos = ant_pos;
        }
        let mut temp_grid = grid;
        temp_grid[ant_pos.1][ant_pos.0] = 3;
        draw_grid(temp_grid, 0);
        println!("{}", depth);
        thread::sleep(std::time::Duration::from_millis(10));
    }

    println!("Max depth reached: {} at {:?}", max_depth_reached, max_depth_reached_pos);

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

    draw_grid(grid, 0);

    let mut new_grid = grid;
    //now loop over the whole thing with a floodfill from the start until the end pos gets filled
    while grid[max_depth_reached_pos.1][max_depth_reached_pos.0] != 2 {//the end will turn from 3 to 4, normal filled cells will go from 0 to 2
        for y in 0..SIZE.1*2+1 {
            for x in 0..SIZE.0*2+1 {
                if grid[y][x] == 0 && (
                    (y > 0 && grid[y-1][x] == 2) ||
                    (y < SIZE.1*2 && grid[y+1][x] == 2) ||
                    (x > 0 && grid[y][x-1] == 2) ||
                    (x < SIZE.0*2 && grid[y][x+1] == 2)
                ) {
                    new_grid[y][x] = 2;
                }
            }
        }
        grid = new_grid;
        draw_grid(grid, 1);
        thread::sleep(std::time::Duration::from_millis(10));
    }
    grid[max_depth_reached_pos.1][max_depth_reached_pos.0] = 3;
    grid[START_POS.1+1][START_POS.0+1] = 3;

    //now loop over again, this time setting any cells with one cardinal neighbours set to 2, to 0
    new_grid = grid;
    loop {
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
        grid = new_grid;
        draw_grid(grid, 2);
        thread::sleep(std::time::Duration::from_millis(20));
    }

    grid[max_depth_reached_pos.1][max_depth_reached_pos.0] = 2;

    draw_grid(grid, 1);
    thread::sleep(std::time::Duration::from_millis(500));
    draw_grid(grid, 3);
    thread::sleep(std::time::Duration::from_millis(500));
    draw_grid(grid, 1);
    thread::sleep(std::time::Duration::from_millis(500));
    draw_grid(grid, 3);
    thread::sleep(std::time::Duration::from_millis(500));

    //now, animate an ant (represented by a cell set to 4) moving from the start to the end
    let mut ant_pos: (usize, usize) = (START_POS.0+1, START_POS.1+1);
    //the algorithm will consist of the ant moving to an adjacent cell that is set to 2, setting the initial cell to 0, and repeating until the end is reached
    while ant_pos != max_depth_reached_pos {
        let prev_ant_pos = ant_pos;
        if grid[ant_pos.1][ant_pos.0+1] >= 2 {
            ant_pos.0 += 1;
        } else if grid[ant_pos.1+1][ant_pos.0] >= 2 {
            ant_pos.1 += 1;
        } else if grid[ant_pos.1][ant_pos.0-1] >= 2 {
            ant_pos.0 -= 1;
        } else if grid[ant_pos.1-1][ant_pos.0] >= 2 {
            ant_pos.1 -= 1;
        }
        grid[prev_ant_pos.1][prev_ant_pos.0] = 0;
        grid[ant_pos.1][ant_pos.0] = 4;
        draw_grid(grid, 3);
        thread::sleep(std::time::Duration::from_millis(10));
    }
    
}
