// Import the necessary modules.
use macroquad::prelude::*;
use crate::KeyCode::{C, Down, Enter, Escape, Left, R, Right, Up};
use r::{Rng, thread_rng};

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
enum Spawn{
    EAST,
    WEST,
    NORTH,
    SOUTH,
    ALLGREEN,
}

// Define a structure representing a traffic light.
struct TrafficLight {
    color: Color,
    position:Spawn,
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
    spawninglocation : Spawn,
    passed_intersection : bool,
    newdirection: Spawn,
    wait: bool,
}


#[macroquad::main("Traffic Simulation Program Rust")]
async fn main() {
    let mut car_id: u64 = 0;// Initialize the ID for the cars
    let mut check_car:usize= 0;// Initialize a variable to check if new cars have been added
    let mut lights:Vec<Color> = vec![];// Create a vector to store the color of the traffic lights
    let mut all_cars: Vec<Car> = vec![];// Create an empty vector to store all cars
    let mut  newspan = Spawn::ALLGREEN;// Initialize the traffic light pattern
    let mut all_traffic_lights : Vec<TrafficLight> = vec![];// Create an empty vector to store all traffic lights


    lights.push(GREEN);// Add the green color to the top left trafic lights
    lights.push(GREEN);// Add the green color to the top right trafic lights
    lights.push(GREEN);//  Add the green color to the bottom right trafic lights
    lights.push(GREEN);// Add the green color to the bottom left trafic lights

    loop {
        // Clear the background with a green color
        clear_background(DARKBROWN);
        
        // Get the mouse position
        let (mouse_x, mouse_y) = mouse_position();
        // Draw the mouse position text
        draw_text(format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(), mouse_x, mouse_y, 25.0, BLACK);
        // Draw the mouse position text
        draw_text(format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(), 0.0, 0.0, 25.0, BLACK);
        
        road();// Draw the road


        
        //  draw_rectangle(position.x, 420.0, 80.0, 80.0, YELLOW);
        // Check for user input and spawn cars
        // Check if a key has been pressed
        if let Some(key) = get_last_key_pressed() {
            // Check if the up arrow has been pressed
            if key == Up {
                // Add a new car coming from the north
                all_cars.push(Car::new(Spawn::NORTH,  car_id as u64));
                // Increment the car ID
                car_id +=1 as u64;
            };

            // Check if the down arrow has been pressed
            if key == Down {
                // Add a new car coming from the south
                all_cars.push(Car::new(Spawn::SOUTH,  car_id as u64));
                // Increment the car ID
                car_id +=1 as u64;
            };

            // Check if the right arrow has been pressed
            if key == Right {
                // Add a new car coming from the west
                all_cars.push(Car::new(Spawn::WEST,  car_id as u64));
                // Increment the car ID
                car_id +=1 as u64;
            };

            // Check if the left arrow has been pressed
            if key == Left {
                // Add a new car coming from the east
                all_cars.push(Car::new(Spawn::EAST,  car_id as u64));
                // Increment the car ID
                car_id +=1 as u64;
            };
        }
         
        // Draw traffic lights
        
        // Draw the traffic light for the west direction (LEFT DRIVERS)
        draw_circle(590.0, 590.0, 10.0, lights[3]);
        
        // Draw the traffic light for the south direction (RIGHT DRIVERS)
        draw_circle(820.0, 390.0, 10.0, lights[1]);
        
        // Draw the traffic light for the east direction (BOTTOM DRIVERS)
        draw_circle(810.0, 605.0, 10.0, lights[2]);
        
        // Draw the traffic light for the north direction (TOP DRIVERS)
        draw_circle(590.0, 390.0, 10.0, lights[0]);

        // Update traffic lights
        if all_cars.len() > check_car{
           (check_car,newspan) = trafficlights(&mut all_cars, check_car);
           println!("newspan: {:?}", newspan);
           if newspan == Spawn::NORTH{
            lights[0] = GREEN;
            lights[1] = RED;
            lights[2] = RED;
            lights[3] = RED;
           }else if newspan == Spawn::WEST{
            lights[0] = RED;
            lights[1] = GREEN;
            lights[2] = RED;
            lights[3] = RED;
           }else if newspan == Spawn::SOUTH{
            lights[0] = RED;
            lights[1] = RED;
            lights[2] = GREEN;
            lights[3] = RED;
           }else if newspan == Spawn::EAST{
            lights[0] = RED;
            lights[1] = RED;
            lights[2] = RED;
            lights[3] = GREEN;
           }
           check_car +=1 as usize;
        }
        for mut onecar in all_cars.iter_mut(){
            if onecar.spawninglocation != newspan {
                onecar.wait = true;
            }else if newspan == Spawn::ALLGREEN {
                onecar.wait = false;
            }else {
                onecar.wait = false;
            }
        }
        for onecar in all_cars.iter_mut(){

                onecar.drive_car();

        }

        next_frame().await
    }
}

fn road(){
    let screen_width = screen_width();
    let screen_height = screen_height();
    {
        // center -> up
        draw_line(620.0, 0.0, 620.0, 420.0, 1.0, YELLOW);//top left line
        draw_line(700.0, 0.0, 700.0, 420.0, 1.0, WHITE);//top middle line
        draw_line(780.0, 0.0, 780.0, 420.0, 1.0, YELLOW);// top right line

    }
   
    {
        // center -> down
        draw_line(620.0, 580.0, 620.0, screen_height, 1.0, RED);//bottom left line
        draw_line(700.0, 580.0, 700.0, screen_height, 1.0, WHITE);//bottom middle line
        draw_line(780.0, 580.0, 780.0, screen_height, 1.0, RED);// bottom right line
    }
    {
        // center -> left
        draw_line(0.0, 420.0, 620.0, 420.0, 1.0, BLUE);//upper-left line
        draw_line(0.0, 500.0, 620.0, 500.0, 1.0, WHITE);//middle-left line
        draw_line(0.0, 580.0, 620.0, 580.0, 1.0, BLUE);//bottom-left line
    }
    {
        // center -> right
        draw_line(780.0, 420.0, screen_width, 420.0, 1.0, GREEN);// upper-right line
        draw_line(780.0, 500.0, screen_width, 500.0, 1.0, WHITE);// middle-right line
        draw_line(780.0, 580.0, screen_width, 580.0, 1.0, GREEN);// bottom-right line
    }
    {
        //  draw a rectangle for the road color (top to bottom)
        draw_rectangle(620.0, 0.0, 780.0 - 620.0, screen_height, Color::new(0.0, 0.4, 0.4, 0.2)); //top to bottom road color
       //  draw a rectangle for the road color (sideways)
       draw_rectangle(0.0, 580.0, screen_width, 460.0 - 620.0, Color::new(0.0, 0.4, 0.4, 0.2)); //left to left road color
    }
}

// Define a struct named "Car"
impl Car {

    // Define a new constructor function for the "Car" struct
    pub fn new(spawninglocation:Spawn, car_id:u64 ) -> Car {
        // Create a new random number generator
        let mut rng = thread_rng();
        // Generate a random number between 0 and 2 (inclusive)
        let selected_colour = match rng.gen_range(0..3) {
            // If the random number is 0, set the color to red
            0 => { Colour::RED }
            // If the random number is 1, set the color to blue
            1 => { Colour::BLUE }
            // If the random number is 2, set the color to green
            2 => { Colour::GREEN }
            // This branch should never be reached, but it's included to make the compiler happy
            _ => { Colour::GREEN }
        };

        // Define a variable named "color"
        let color: Color;

        // Set the "color" variable based on the selected color
        match selected_colour {
            Colour::RED  => {color = RED}
            Colour::BLUE  => {color = BLUE}
            Colour::GREEN  => {color = GREEN}
        }

        // Define a variable named "direction"
        let direction = match selected_colour {
            // If the selected color is red, set the direction to left
            Colour::RED  => {Direction::LEFT}
            // If the selected color is blue, set the direction to right
            Colour::BLUE  => {Direction::RIGHT}
            // If the selected color is green, set the direction to straight
            Colour::GREEN  => {Direction::STRAIGHT}
        };

        // Define variables named "x" and "y" (WHICH IS USED TO KEEP THE CAR IN THE CENTRE OF THE ROAD)
        let (x, y) = match spawninglocation {
            // If the spawning location is north, set the x and y coordinates accordingly
            Spawn::NORTH => { (625.0, ( 00.0)) }
            // If the spawning location is south, set the x and y coordinates accordingly
            Spawn::SOUTH=> { (705.0, screen_height()-10.0) }
            // If the spawning location is west, set the x and y coordinates accordingly
            Spawn::WEST => { ((screen_width()-10.0 ), 425.0 /*y: ((screen_height * 0.65 - screen_height / 2.0) / 2.0 + screen_height / 2.0) - (screen_height / 16.0) / 2.0*/) }
            // If the spawning location is east, set the x and y coordinates accordingly
            Spawn::EAST => { (0.0, 505.0) }
            // If the spawning location is all green, set the x and y coordinates to (0, 0)
            Spawn::ALLGREEN => (0.0, 0.0)
        };

        // Create a new Car object with the given properties
        let xnewcar = Car {
            id : car_id,
            height: 70.0,
            width: 70.0,
            spawninglocation,
            direction,
            position: CarPosition{x,y},
            color,
            passed_intersection: false,
            newdirection: Spawn::EAST,
            wait:false,
        };

        // Print the new car's properties for debugging purposes
        println!("{:?}", xnewcar);

        // Return the new car object
        return xnewcar;

        // Car {
        //     id : car_id,
        //     height: 80.0,
        //     width: 80.0,
        //     spawninglocation,
        //     direction:Direction::UP, // change this
        //     position: CarPosition{x,y},
        //     color,
        // }

    }


    fn drive_car(&mut self) {
        
        // Draw rectangle using the car's position, width, height, and color
        draw_rectangle(self.position.x, self.position.y, self.width, self.height, self.color);
        // println!("drawcar: {},{},{},{},{:?}", self.position.x, self.position.y, self.width, self.height, self.color);
        // self.turn();
        
        // Check if the car is waiting at a specific location before moving
        if self.wait && self.spawninglocation == Spawn::EAST && self.position.x == 540.00 {
            // Do nothing if the car is waiting at this location
        } else if self.wait && self.spawninglocation == Spawn::WEST && self.position.x == 780.00 {
            // Do nothing if the car is waiting at this location
        } else if self.wait && self.spawninglocation == Spawn::NORTH && self.position.y == 340.00 {
            // Do nothing if the car is waiting at this location
        } else if self.wait && self.spawninglocation == Spawn::SOUTH && self.position.y == 580.00 {
            // Do nothing if the car is waiting at this location
        } else {
            // Move the car based on its spawning location, position, and color
            if self.spawninglocation == Spawn::EAST {
                if self.position.x == 699.0 && self.position.y == 500.0 && !self.passed_intersection && self.color == RED {
                    // Update the car's spawning location and passed intersection flag
                    self.passed_intersection = true;
                    self.spawninglocation = Spawn::SOUTH;
                } else if self.position.x == 619.0 && self.position.y == 500.0 && !self.passed_intersection && self.color == BLUE {
                    // Update the car's spawning location and passed intersection flag
                    self.spawninglocation = Spawn::NORTH;
                    self.passed_intersection = true;
                }
                // Move the car to the right
                self.position.x += 10.0;
            }
            if self.spawninglocation == Spawn::WEST {
                if self.position.x == 621.0 && self.position.y == 420.0 && !self.passed_intersection && self.color == RED {
                    // Update the car's spawning location and passed intersection flag
                    self.passed_intersection = true;
                    self.spawninglocation = Spawn::NORTH;
                } else if self.position.x == 701.0 && self.position.y == 420.0 && !self.passed_intersection && self.color == BLUE {
                    // Update the car's spawning location and passed intersection flag
                    self.spawninglocation = Spawn::SOUTH;
                    self.passed_intersection = true;
                }
                // Move the car to the left
                self.position.x -= 10.0;
            }
            if self.spawninglocation == Spawn::NORTH {
                if self.position.x == 620.0 && self.position.y == 499.0 && !self.passed_intersection && self.color == RED {
                    // Update the car's spawning location and passed intersection flag
                    self.passed_intersection = true;
                    self.spawninglocation = Spawn::EAST;
                } else if self.position.x == 620.0 && self.position.y == 419.0 && !self.passed_intersection && self.color == BLUE {
                    // Update the car's spawning location and passed intersection flag
                    self.spawninglocation = Spawn::WEST;
                    self.passed_intersection = true;
                }
                // Move the car up
                self.position.y += 10.0;
            }
            if self.spawninglocation == Spawn::SOUTH {
                // Check if the car is at the intersection, has not passed it, and has the correct color.
                if self.position.x==700.0 && self.position.y == 421.0 && !self.passed_intersection && self.color == RED{
                    // If the car meets the above conditions, mark that it has passed the intersection and change its direction to West.
                    self.passed_intersection = true;
                    self.spawninglocation = Spawn::WEST;
                } else if self.position.x==700.0 && self.position.y == 501.0 && !self.passed_intersection && self.color == BLUE {
                    // If the car meets the above conditions, mark that it has passed the intersection and change its direction to East.
                    self.spawninglocation = Spawn::EAST;
                    self.passed_intersection = true;
                }
                // Move the car 10 units in the negative y-direction (south).
                self.position.y -= 10.0;
            }
        }
    }
}

// This function updates the traffic lights and returns a tuple with the index of 
// the next car to be processed and the current spawning location.
fn trafficlights(all_cars: &mut Vec<Car>,mut checkcar: usize) -> (usize, Spawn){
    
    // Print the length of the all_cars vector and the index of the current car being processed.
    println!("allcar len: {}, checkcar: {}",all_cars.len(), checkcar);
    
    // Print the current car being processed.
    println!("{:?}",all_cars[checkcar]);

    // If the current car has passed the intersection, move to the next car and set the 
    // traffic lights to all green.
    if all_cars[checkcar].passed_intersection == true {
        checkcar+=1 as usize;
        (checkcar, Spawn::ALLGREEN)

    }else {

        // Otherwise, set the traffic lights according to the current car's spawning location.
        draw_circle(590.0, 590.0, 10.0, RED);
        draw_circle(820.0, 390.0, 10.0, RED);
        draw_circle(810.0, 605.0, 10.0, RED);
        draw_circle(590.0, 390.0, 10.0, RED);
        
        match all_cars[checkcar].spawninglocation  {
            Spawn::NORTH => {draw_circle(590.0, 390.0, 10.0, GREEN)},
            Spawn::SOUTH=> {draw_circle(810.0, 605.0, 10.0, GREEN)},
            Spawn::WEST =>{draw_circle(820.0, 390.0, 10.0, GREEN)},
            Spawn::EAST => {draw_circle(590.0, 590.0, 10.0, GREEN)},
            Spawn::ALLGREEN => {draw_circle(100.0, 100.0, 10.0, GREEN);
                draw_circle(130.0, 100.0, 10.0, GREEN);
                draw_circle(160.0, 100.0, 10.0, GREEN);
                draw_circle(190.0, 100.0, 10.0, GREEN)
            },
        };
        //    checkcar +=1 as  usize;
        // Return the index of the current car and the current spawning location.
        (checkcar, all_cars[checkcar].spawninglocation.clone())
    }
}

