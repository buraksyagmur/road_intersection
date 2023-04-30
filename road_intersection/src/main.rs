use macroquad::prelude::*;
use crate::KeyCode::{C, Down, Enter, Escape, Left, R, Right, Up};
use r::{Rng, thread_rng};
#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    STRAIGHT,
    RIGHT,
    LEFT,
}
#[derive(Clone,Debug)]
enum Colour {
    RED,
    BLUE,
    GREEN,
}
#[derive(Clone,Debug, PartialEq)]
enum Spawn{
    EAST,
    WEST,
    NORTH,
    SOUTH,
    ALLGREEN,
}
struct TrafficLight {
    color: Color,
    position:Spawn ,
}
#[derive(Clone, Debug, PartialEq)]
struct CarPosition {
    x: f32,
    y: f32,
}
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


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut all_cars: Vec<Car> = vec![];
    let mut all_traffic_lights : Vec<TrafficLight> = vec![];
    let mut car_id: u64 = 0;
    let mut check_car:usize= 0;
    let mut  newspan = Spawn::ALLGREEN;
    let mut lights:Vec<Color> = vec![];
    lights.push(GREEN);
    lights.push(GREEN);
    lights.push(GREEN);
    lights.push(GREEN);
    loop {
        clear_background(WHITE);
        let (mouse_x, mouse_y) = mouse_position();
        draw_text(format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(), mouse_x, mouse_y, 15.0, DARKGRAY);
        draw_text(format!("X: {}, Y:{}", mouse_x, mouse_y).as_str(), 0.0, 0.0, 15.0, DARKGRAY);
        road();

        //  draw_rectangle(position.x, 420.0, 80.0, 80.0, YELLOW);
        if let Some(key) = get_last_key_pressed() {
            if key == Up {
             all_cars.push(Car::new(Spawn::NORTH,  car_id as u64));
             car_id +=1 as u64;

            };
            if key == Down {
                all_cars.push(Car::new(Spawn::SOUTH,  car_id as u64));
                car_id +=1 as u64;
            };
            if key == Right {
                all_cars.push(Car::new(Spawn::WEST,  car_id as u64));
                car_id +=1 as u64;
            };
            if key == Left {
                all_cars.push(Car::new(Spawn::EAST,  car_id as u64));
                car_id +=1 as u64;
            };
        }
        draw_circle(590.0, 590.0, 10.0, lights[3]);
        draw_circle(820.0, 390.0, 10.0, lights[1]);
        draw_circle(810.0, 605.0, 10.0, lights[2]);
        draw_circle(590.0, 390.0, 10.0, lights[0]);
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
        for mut onecar in all_cars.iter_mut(){

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
        draw_line(620.0, 0.0, 620.0, 420.0, 1.0, GRAY);
        draw_line(700.0, 0.0, 700.0, 420.0, 1.0, GRAY);
        draw_line(780.0, 0.0, 780.0, 420.0, 1.0, GRAY);
    }
    {
        // center -> down
        draw_line(620.0, 580.0, 620.0, screen_height, 1.0, GRAY);
        draw_line(700.0, 580.0, 700.0, screen_height, 1.0, GRAY);
        draw_line(780.0, 580.0, 780.0, screen_height, 1.0, GRAY);
    }
    {
        // center -> left
        draw_line(0.0, 420.0, 620.0, 420.0, 1.0, GRAY);
        draw_line(0.0, 500.0, 620.0, 500.0, 1.0, GRAY);
        draw_line(0.0, 580.0, 620.0, 580.0, 1.0, GRAY);
    }
    {
        // center -> right
        draw_line(780.0, 420.0, screen_width, 420.0, 1.0, GRAY);
        draw_line(780.0, 500.0, screen_width, 500.0, 1.0, GRAY);
        draw_line(780.0, 580.0, screen_width, 580.0, 1.0, GRAY);
    }
}
impl Car {
    pub fn new(spawninglocation:Spawn, car_id:u64 ) -> Car{
        let mut rng = thread_rng();
        let selected_colour = match rng.gen_range(0..3) {
            0 => { Colour::RED }
            1 => { Colour::BLUE }
            2 => { Colour::GREEN }
            _ => { Colour::GREEN }
        };
        let color: Color;
        match selected_colour  {
              Colour::RED  => {color = RED}
              Colour::BLUE  => {color = BLUE}
              Colour::GREEN  => {color = GREEN}
        }
        let direction = match selected_colour {
            Colour::RED  => {Direction::LEFT}
            Colour::BLUE  => {Direction::RIGHT}
            Colour::GREEN  => {Direction::STRAIGHT}
        };
        let (x, y) = match spawninglocation {
            Spawn::NORTH => { (620.0, ( 00.0)) }
            Spawn::SOUTH=> { (700.0, screen_height()-10.0) }
            Spawn::WEST => { ((screen_width()-10.0 ), 420.0 /*y: ((screen_height * 0.65 - screen_height / 2.0) / 2.0 + screen_height / 2.0) - (screen_height / 16.0) / 2.0*/) }
            Spawn::EAST => { (0.0, 500.0) }
            Spawn::ALLGREEN => (0.0, 0.0)
        };
        let xnewcar =Car {
            id : car_id,
            height: 80.0,
            width: 80.0,
            spawninglocation,
            direction,
            position: CarPosition{x,y},
            color,
            passed_intersection: false,
            newdirection: Spawn::EAST,
            wait:false,
        };
        println!("{:?}", xnewcar);
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
        draw_rectangle(self.position.x, self.position.y, self.width, self.height, self.color);
        // println!("drawcar: {},{},{},{},{:?}", self.position.x, self.position.y, self.width, self.height, self.color);
        // self.turn();
        if self.wait && self.spawninglocation == Spawn::EAST && self.position.x == 540.00{

        }else  if self.wait && self.spawninglocation == Spawn::WEST && self.position.x == 780.00{

        }else  if self.wait && self.spawninglocation == Spawn::NORTH && self.position.y == 340.00{

        }else if self.wait && self.spawninglocation == Spawn::SOUTH && self.position.y == 580.00{

        }else {
            if self.spawninglocation == Spawn::EAST {
                if self.position.x==699.0 && self.position.y == 500.0 && !self.passed_intersection && self.color == RED{
                 self.passed_intersection = true;
                    self.spawninglocation = Spawn::SOUTH;
                }else if self.position.x==619.0 && self.position.y == 500.0 && !self.passed_intersection && self.color == BLUE {
                        self.spawninglocation = Spawn::NORTH;
                        self.passed_intersection = true;
                    }
                    self.position.x += 10.0;
            }
            if self.spawninglocation == Spawn::WEST {
                if self.position.x==621.0 && self.position.y == 420.0 && !self.passed_intersection && self.color == RED{
                    self.passed_intersection = true;
                    self.spawninglocation = Spawn::NORTH;
                }else if self.position.x==701.0 && self.position.y == 420.0 && !self.passed_intersection && self.color == BLUE {
                        self.spawninglocation = Spawn::SOUTH;
                        self.passed_intersection = true;
                    }
                self.position.x -= 10.0;
            }
            if self.spawninglocation == Spawn::NORTH {
                if self.position.x==620.0 && self.position.y == 499.0 && !self.passed_intersection && self.color == RED{
                    self.passed_intersection = true;
                    self.spawninglocation = Spawn::EAST;
                }else if self.position.x==620.0 && self.position.y == 419.0 && !self.passed_intersection && self.color == BLUE{
                    self.spawninglocation = Spawn::WEST;
                    self.passed_intersection = true;
                }
                self.position.y += 10.0;
            }
            if self.spawninglocation == Spawn::SOUTH {
                if self.position.x==700.0 && self.position.y == 421.0 && !self.passed_intersection && self.color == RED{
                    self.passed_intersection = true;
                    self.spawninglocation = Spawn::WEST;
                }else if self.position.x==700.0 && self.position.y == 501.0 && !self.passed_intersection && self.color == BLUE {
                        self.spawninglocation = Spawn::EAST;
                        self.passed_intersection = true;
                    }
                    self.position.y -= 10.0;
                }
            }

    }
}
fn trafficlights(all_cars: &mut Vec<Car>,mut checkcar: usize) -> (usize, Spawn){
    println!("allcar len: {}, checkcar: {}",all_cars.len(), checkcar);
    println!("{:?}",all_cars[checkcar]);
    if all_cars[checkcar].passed_intersection == true {
        checkcar+=1 as usize;
        (checkcar, Spawn::ALLGREEN)
    }else {
        draw_circle(590.0, 590.0, 10.0, RED);
        draw_circle(820.0, 390.0, 10.0, RED);
        draw_circle(810.0, 605.0, 10.0, RED);
        draw_circle(590.0, 390.0, 10.0, RED);
        match all_cars[checkcar].spawninglocation  {
        Spawn::NORTH => {draw_circle(590.0, 390.0, 10.0, GREEN)},
        Spawn::SOUTH=> {draw_circle(810.0, 605.0, 10.0, GREEN)},
        Spawn::WEST =>{draw_circle(820.0, 390.0, 10.0, GREEN)},
        Spawn::EAST => {draw_circle(590.0, 590.0, 10.0, GREEN)},
        Spawn::ALLGREEN => {draw_circle(100.0, 100.0, 10.0, GREEN);  draw_circle(130.0, 100.0, 10.0, GREEN);draw_circle(160.0, 100.0, 10.0, GREEN); draw_circle(190.0, 100.0, 10.0, GREEN)},
       };
    //    checkcar +=1 as  usize;
       (checkcar, all_cars[checkcar].spawninglocation.clone())
    }
}

