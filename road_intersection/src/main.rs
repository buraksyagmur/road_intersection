// Import the necessary modules.
use macroquad::prelude::*;
// use macroquad::window::{self, Conf, Window};
// use macroquad::window::is_window_ready;
use macroquad::Window;
use crate::KeyCode::{Down, Enter, Escape, Left, Right, Up, R};
use chrono::{DateTime, Duration, Local};
use core::panic;
use macroquad::prelude::*;
use macroquad::prelude::*;
use r::{thread_rng, Rng};
use std::thread;
use std::thread::sleep;

// Define an enumeration of possible directions.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    STRAIGHT,
    RIGHT,
    LEFT,
}

// Define an enumeration of possible colors.
#[derive(Clone, Debug)]
enum Colour {
    RED,
    BLUE,
    GREEN,
}

// Define an enumeration of possible spawn locations.
#[derive(Clone, Debug, PartialEq)]
enum Spawn {
    EAST,
    WEST,
    NORTH,
    SOUTH,
    ALLGREEN,
}

// Define a structure representing a traffic light.
struct TrafficLight {
    color: Color,
    position: Spawn,
}

// Define a structure representing the position of a car.
#[derive(Clone, Debug)]
struct CarPosition {
    x: f32,
    y: f32,
}

// Define a structure representing a car.
#[derive(Clone, Debug)]
struct Car {
    id: u64,
    height: f32,
    width: f32,
    direction: Direction,
    position: CarPosition,
    color: Color,
    spawninglocation: Spawn,
    passed_intersection: bool,
    newdirection: bool,
    wait: bool,
    emergency_stop: bool,
    where_are_you_from: Spawn,
    passing_time: DateTime<Local>,
    arrived: bool,
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Road intersection".to_owned(),
        // window_height: 1600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut paused = false;
    let mut pause_start_time = 0.0;
    let screen_height = screen_height();
    let screen_width = screen_width();
    println!("height{}, width {}", screen_height, screen_width);
    let road_width = screen_width * 0.2;
    let added_value = road_width / 2.0;
    let mut last_spawn_time_up = get_time();
    let mut last_spawn_time_down = get_time();
    let mut last_spawn_time_left = get_time();
    let mut last_spawn_time_right = get_time();
    let mut last_spawn_time_r = get_time();
    let center_x = screen_width / 2.0;
    let center_y = screen_height / 2.0;
    let mut all_cars: Vec<Car> = vec![];
    let mut all_traffic_lights: Vec<TrafficLight> = vec![];
    let mut car_id: u64 = 0;
    let mut check_car: usize = 0;
    let mut newspan = Spawn::ALLGREEN;
    let mut lights: Vec<Color> = vec![];
    let mut first_iteration: bool = false;
    lights.push(GREEN);
    lights.push(GREEN);
    lights.push(GREEN);
    lights.push(GREEN);
    // let handle = next_frame().await;
    // let is_ready = is_fullscreen();
    // if is_ready {
    //     // Do something
    // }
    // handle.await;
    // let _window = macroquad::Window::new("My App", window_conf);
    loop {
        clear_background(WHITE);
       if !paused{ let (mouse_x, mouse_y) = mouse_position();
        draw_text(
            format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(),
            mouse_x,
            mouse_y,
            15.0,
            BLACK,
        );
        draw_text(
            format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(),
            0.0,
            0.0,
            15.0,
            BLACK,
        );
        road();
        //This is a new variable to store the time of the last spawn:


        // Here we create a threshold for the minimum time between spawns (to 0.5 seconds):
        let spawn_throttle_time = 3.0;

        if let Some(key) = get_last_key_pressed() {
            // Check if the up arrow has been pressed
            if get_time() - last_spawn_time_up >= spawn_throttle_time {
            if key == Down {
                // Add a new car coming from the north
                all_cars.push(Car::new(Spawn::NORTH, car_id as u64));
                // Increment the car ID
                car_id += 1 as u64;

                // geting the spawn time
                last_spawn_time_up = get_time();
            };
        }
            // Check if the down arrow has been pressed
            if get_time() - last_spawn_time_down >= spawn_throttle_time {
            if key == Up {
                // Add a new car coming from the south
                all_cars.push(Car::new(Spawn::SOUTH, car_id as u64));
                // Increment the car ID
                car_id += 1 as u64;
                // geting the spawn time
                last_spawn_time_down = get_time();
            };
        }
            // Check if the right arrow has been pressed
            if get_time() - last_spawn_time_right >= spawn_throttle_time {
            if key == Right {
                // Add a new car coming from the west
                all_cars.push(Car::new(Spawn::EAST, car_id as u64));
                // Increment the car ID
                car_id += 1 as u64;
                // geting the spawn time
                last_spawn_time_right = get_time();
            };
        }
            // Check if the left arrow has been pressed
            if get_time() - last_spawn_time_left >= spawn_throttle_time {
            if key == Left {
                // Add a new car coming from the east
                all_cars.push(Car::new(Spawn::WEST, car_id as u64));
                // Increment the car ID
                car_id += 1 as u64;
                // geting the spawn time
                last_spawn_time_left = get_time();
            };
        }
            // implementing the 'r' key for spawning a vehicle from a random direction.
            if get_time() - last_spawn_time_r >= spawn_throttle_time {
            if key == R {
                let mut rng = thread_rng();
                let random_direction = rng.gen_range(0..4);
                let spawn_location = match random_direction {
                    0 => Spawn::NORTH,
                    1 => Spawn::WEST,
                    2 => Spawn::SOUTH,
                    _ => Spawn::EAST,
                };
                // Add a new car coming from the east
                all_cars.push(Car::new(spawn_location, car_id as u64));
                // Increment the car ID
                car_id += 1 as u64;
                // geting the spawn time
                last_spawn_time_r = get_time();
            }
        }
            if key == Escape {
                break;
            }
            if is_key_pressed(KeyCode::P) {
                paused = true;
                pause_start_time = get_time();
            }
        }
        draw_line(
            center_x - added_value,
            center_y - added_value,
            center_x,
            center_y - added_value,
            0.5,
            lights[0],
        );
        draw_line(
            center_x + added_value,
            center_y,
            center_x + added_value,
            center_y - added_value,
            0.5,
            lights[1],
        );
        draw_line(
            center_x,
            center_y + added_value,
            center_x + added_value,
            center_y + added_value,
            0.5,
            lights[2],
        );
        draw_line(
            center_x - added_value,
            center_y + added_value,
            center_x - added_value,
            center_y,
            0.5,
            lights[3],
        );
        if all_cars.len() > check_car {
            (check_car, newspan) = trafficlights(&mut all_cars, check_car);
            println!("another: {:?}, newspan: {:?}", check_car, newspan);
            if newspan == Spawn::NORTH {
                lights[0] = GREEN;
                lights[1] = RED;
                lights[2] = RED;
                lights[3] = RED;
            } else if newspan == Spawn::WEST {
                lights[0] = RED;
                lights[1] = GREEN;
                lights[2] = RED;
                lights[3] = RED;
            } else if newspan == Spawn::SOUTH {
                lights[0] = RED;
                lights[1] = RED;
                lights[2] = GREEN;
                lights[3] = RED;
            } else if newspan == Spawn::EAST {
                lights[0] = RED;
                lights[1] = RED;
                lights[2] = RED;
                lights[3] = GREEN;
            } else if newspan == Spawn::ALLGREEN {
                lights[0] = GREEN;
                lights[1] = GREEN;
                lights[2] = GREEN;
                lights[3] = GREEN;
            }
        }
        for mut onecar in all_cars.iter_mut() {
            if onecar.position.x > screen_width + added_value
                || onecar.position.x < 0.0
                || onecar.position.y > screen_height + added_value
                || onecar.position.y < 0.0
            {
                onecar.arrived = true;
            }
        }
        for mut onecar in all_cars.iter_mut() {
            println!(
                "firstiteration: {}, selectedcar: {:?}",
                first_iteration, onecar
            );
            if onecar.spawninglocation != newspan {
                onecar.wait = true;
            } else {
                onecar.wait = false;
                onecar.emergency_stop = false;
            }
            if newspan == Spawn::ALLGREEN {
                onecar.wait = false;
            }
        }

        for i in 0..all_cars.len() {
            for j in (i + 1)..all_cars.len() {
                if i < j
                    && all_cars[i].spawninglocation == all_cars[j].spawninglocation
                    && (((all_cars[i].position.x - all_cars[j].position.x).abs()
                        < (2.0 * added_value)
                        && all_cars[i].position.y == all_cars[j].position.y)
                        || ((all_cars[i].position.y - all_cars[j].position.y).abs()
                            < (2.0 * added_value)
                            && all_cars[i].position.x == all_cars[j].position.x))
                {
                    println!("reason: {:?}", all_cars[i]);
                    println!("stopped: {:?}", all_cars[j]);
                    all_cars[j].emergency_stop = true;
                }
            }
        }
        for i in 0..all_cars.len() {
            for j in (i + 1)..all_cars.len() {
                if i < j
                    && !all_cars[i].arrived
                    && all_cars[i].spawninglocation == all_cars[j].spawninglocation
                    && all_cars[i].wait == false
                    && all_cars[j].wait == false
                {
                    if i + 1 as usize != j {
                        all_cars[j].wait = true;
                    }
                }
            }
        }
        // for i in 0..all_cars.len() {
        //     for j in (i + 1)..all_cars.len() {
        //         let time_diff: Duration = all_cars[j].passing_time.signed_duration_since(all_cars[i].passing_time);
        //         if i < j
        //             && all_cars[i].where_are_you_from == all_cars[j].where_are_you_from
        //             && all_cars[i].wait == false
        //             && all_cars[j].wait == true && time_diff.num_seconds() > 3 as i64
        //         {
        //             all_cars[j].wait = false;
        //         }
        //     }
        // }

        for mut onecar in all_cars.iter_mut() {
            onecar.drive_car();
        }}else{
            draw_text("Paused", center_x, center_y, 32.0, Color::new(1.0, 1.0, 1.0, 1.0));

            if is_key_pressed(KeyCode::C) {
                paused = false;
            }
        }
        next_frame().await
    }
}
fn road() {
    let screen_height = screen_height();
    let screen_width = screen_width();

    let road_width = screen_width * 0.2;
    let added_value = road_width / 2.0;

    let center_x = screen_width / 2.0;
    let center_y = screen_height / 2.0;

    {
        draw_line(
            center_x + added_value,
            0.0,
            center_x + added_value,
            center_y - added_value,
            1.0,
            GRAY,
        );
        draw_line(center_x, 0.0, center_x, center_y - added_value, 1.0, GRAY);
        draw_line(
            center_x - added_value,
            0.0,
            center_x - added_value,
            center_y - added_value,
            1.0,
            GRAY,
        );
    }
    {
        draw_line(
            center_x + added_value,
            center_y + added_value,
            center_x + added_value,
            screen_height,
            1.0,
            GRAY,
        );
        draw_line(
            center_x,
            center_y + added_value,
            center_x,
            screen_height,
            1.0,
            GRAY,
        );
        draw_line(
            center_x - added_value,
            center_y + added_value,
            center_x - added_value,
            screen_height,
            1.0,
            GRAY,
        );
    }
    {
        draw_line(
            0.0,
            center_y + added_value,
            center_x - added_value,
            center_y + added_value,
            1.0,
            GRAY,
        );
        draw_line(0.0, center_y, center_x - added_value, center_y, 1.0, GRAY);
        draw_line(
            0.0,
            center_y - added_value,
            center_x - added_value,
            center_y - added_value,
            1.0,
            GRAY,
        );
    }
    {
        draw_line(
            center_x + added_value,
            center_y + added_value,
            screen_width,
            center_y + added_value,
            1.0,
            GRAY,
        );
        draw_line(
            center_x + added_value,
            center_y,
            screen_width,
            center_y,
            1.0,
            GRAY,
        );
        draw_line(
            center_x + added_value,
            center_y - added_value,
            screen_width,
            center_y - added_value,
            1.0,
            GRAY,
        );
    }
}
impl Car {
    pub fn new(spawninglocation: Spawn, car_id: u64) -> Car {
        let screen_height = screen_height();
        let screen_width = screen_width();
        let road_width = screen_width * 0.2;
        let added_value = road_width / 2.0;
        let center_x = screen_width / 2.0;
        let center_y = screen_height / 2.0;
        let mut rng = thread_rng();
        let selected_colour = match rng.gen_range(0..3) {
            0 => Colour::RED,
            1 => Colour::BLUE,
            2 => Colour::GREEN,
            _ => Colour::GREEN,
        };
        let color: Color;
        match selected_colour {
            Colour::RED => color = RED,
            Colour::BLUE => color = BLUE,
            Colour::GREEN => color = GREEN,
        }
        let direction = match selected_colour {
            Colour::RED => Direction::LEFT,
            Colour::BLUE => Direction::RIGHT,
            Colour::GREEN => Direction::STRAIGHT,
        };
        let (x, y) = match spawninglocation {
            Spawn::NORTH => (center_x - added_value, 0.0),
            Spawn::SOUTH => (center_x, screen_height),
            Spawn::WEST => (screen_width - added_value, center_y - added_value),
            Spawn::EAST => (0.0, center_y),
            Spawn::ALLGREEN => (0.0, 0.0),
        };
        let now = Local::now();
        let clonespawn = spawninglocation.clone();
        let xnewcar = Car {
            id: car_id,
            height: added_value,
            width: added_value,
            spawninglocation,
            direction,
            position: CarPosition { x, y },
            color,
            passed_intersection: false,
            newdirection: false,
            wait: false,
            emergency_stop: false,
            where_are_you_from: clonespawn,
            passing_time: now,
            arrived: false,
        };
        return xnewcar;
    }
    fn drive_car(&mut self) {
        let screen_height = screen_height();
        let screen_width = screen_width();
        let road_width = screen_width * 0.2;
        let added_value = road_width / 2.0;
        let center_x = screen_width / 2.0;
        let center_y = screen_height / 2.0;
        let velo: f32 = 4.0;
        let wait_dur = (added_value / velo) as u64;

        draw_rectangle(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
            self.color,
        );

        if self.wait
            && self.spawninglocation == Spawn::EAST
            && self.position.x == center_x - (2.5 * added_value)
            && !self.newdirection
        {
        } else if self.wait
            && self.spawninglocation == Spawn::WEST
            && self.position.x == center_x + (1.5 * added_value)
            && !self.newdirection
        {
        } else if self.wait
            && self.spawninglocation == Spawn::NORTH
            && self.position.y == center_y - (2.5 * added_value)
            && !self.newdirection
        {
        } else if self.wait
            && self.spawninglocation == Spawn::SOUTH
            && self.position.y == center_y + (1.5 * added_value)
            && !self.newdirection
        {
        } else if self.emergency_stop {
        } else {
            if self.spawninglocation == Spawn::EAST {
                if self.newdirection
                    && !self.passed_intersection
                    && self.position.x == center_x - (2.0 * added_value)
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.newdirection
                    && !self.passed_intersection
                    && self.position.x == center_x + (2.0 * added_value)
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.position.x == center_x && !self.passed_intersection && !self.newdirection {
                    if self.color == RED {
                        self.spawninglocation = Spawn::SOUTH;
                        self.newdirection = true;
                    } else if self.color == GREEN && !self.newdirection {
                        self.newdirection = true;
                    }
                } else if self.position.x == center_x - added_value
                    && !self.passed_intersection
                    && self.color == BLUE
                {
                    self.spawninglocation = Spawn::NORTH;
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                } else {
                    self.position.x += velo;
                }
            }
            if self.spawninglocation == Spawn::WEST {
                if self.newdirection
                    && !self.passed_intersection
                    && self.position.x == center_x - (2.0 * added_value)
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.newdirection
                    && !self.passed_intersection
                    && self.position.x == center_x + (2.0 * added_value)
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.position.x == center_x - added_value
                    && !self.passed_intersection
                    && !self.newdirection
                {
                    if self.color == RED {
                        self.spawninglocation = Spawn::NORTH;
                        self.newdirection = true;
                    } else if self.color == GREEN && !self.newdirection {
                        self.newdirection = true;
                    }
                } else if self.position.x == center_x
                    && !self.passed_intersection
                    && self.color == BLUE
                {
                    self.spawninglocation = Spawn::SOUTH;
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                } else {
                    self.position.x -= velo;
                }
            }
            if self.spawninglocation == Spawn::NORTH {
                if self.newdirection
                    && !self.passed_intersection
                    && self.position.y == center_y + (2.0 * added_value)
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.newdirection
                    && !self.passed_intersection
                    && self.position.y == center_y - (2.0 * added_value)
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.position.y == center_y && !self.passed_intersection && !self.newdirection {
                    if self.color == RED {
                        self.spawninglocation = Spawn::EAST;
                        self.newdirection = true;
                    } else if self.color == GREEN && !self.newdirection {
                        self.newdirection = true;
                    } else if self.color == RED && self.newdirection {
                    }
                } else if self.position.y == center_y - added_value
                    && !self.passed_intersection
                    && self.color == BLUE
                {
                    self.spawninglocation = Spawn::WEST;
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                } else {
                    self.position.y += velo;
                }
            }
            if self.spawninglocation == Spawn::SOUTH {
                if self.newdirection
                    && self.position.y == center_y - (2.0 * added_value)
                    && !self.passed_intersection
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.newdirection
                    && self.position.y == center_y + (2.0 * added_value)
                    && !self.passed_intersection
                {
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                }
                if self.position.y == center_y - added_value
                    && !self.passed_intersection
                    && !self.newdirection
                {
                    if self.color == RED {
                        self.spawninglocation = Spawn::WEST;
                        self.newdirection = true;
                    } else if self.color == GREEN && !self.newdirection {
                        self.newdirection = true;
                    } else if self.color == RED && self.newdirection {
                    }
                } else if self.position.y == center_y
                    && !self.passed_intersection
                    && self.color == BLUE
                {
                    self.spawninglocation = Spawn::EAST;
                    self.passed_intersection = true;
                    self.passing_time = Local::now();
                } else {
                    self.position.y -= velo;
                }
            }
        }
    }
}
fn trafficlights(all_cars: &mut Vec<Car>, mut checkcar: usize) -> (usize, Spawn) {
    let screen_height = screen_height();
    let screen_width = screen_width();
    let road_width = screen_width * 0.2;
    let added_value = road_width / 2.0;
    let center_x = screen_width / 2.0;
    let center_y = screen_height / 2.0;

    let center_x = screen_width / 2.0;
    let center_y = screen_height / 2.0;
    println!("problem:{:?}, id:{}", all_cars[checkcar], checkcar);
    if all_cars[checkcar].passed_intersection == true {
        checkcar += 1 as usize;
    }
    if checkcar as usize == all_cars.len() {
        return (checkcar, Spawn::ALLGREEN);
    }
    draw_line(
        center_x - added_value,
        center_y - added_value,
        center_x,
        center_y - added_value,
        0.5,
        RED,
    );
    draw_line(
        center_x + added_value,
        center_y,
        center_x + added_value,
        center_y - added_value,
        0.5,
        RED,
    );
    draw_line(
        center_x,
        center_y + added_value,
        center_x + added_value,
        center_y + added_value,
        0.5,
        RED,
    );
    draw_line(
        center_x - added_value,
        center_y + added_value,
        center_x - added_value,
        center_y,
        0.5,
        RED,
    );
    match all_cars[checkcar].spawninglocation {
        Spawn::NORTH => {
            draw_line(
                center_x - added_value,
                center_y - added_value,
                center_x,
                center_y - added_value,
                0.5,
                GREEN,
            );
        }
        Spawn::SOUTH => {
            draw_line(
                center_x,
                center_y + added_value,
                center_x + added_value,
                center_y + added_value,
                0.5,
                GREEN,
            );
        }
        Spawn::WEST => {
            draw_line(
                center_x + added_value,
                center_y,
                center_x + added_value,
                center_y - added_value,
                0.5,
                GREEN,
            );
        }
        Spawn::EAST => {
            draw_line(
                center_x - added_value,
                center_y + added_value,
                center_x - added_value,
                center_y,
                0.5,
                GREEN,
            );
        }
        Spawn::ALLGREEN => {
            draw_line(
                center_x - added_value,
                center_y - added_value,
                center_x,
                center_y - added_value,
                0.5,
                GREEN,
            );
            draw_line(
                center_x + added_value,
                center_y,
                center_x + added_value,
                center_y - added_value,
                0.5,
                GREEN,
            );
            draw_line(
                center_x,
                center_y + added_value,
                center_x + added_value,
                center_y + added_value,
                0.5,
                GREEN,
            );
            draw_line(
                center_x - added_value,
                center_y + added_value,
                center_x - added_value,
                center_y,
                0.5,
                GREEN,
            );
        }
    };

    (checkcar, all_cars[checkcar].spawninglocation.clone())
}
