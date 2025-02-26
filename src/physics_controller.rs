use std::time::{Duration, SystemTime};

use crate::rect_collider::RectCollider;
use crate::plate_controller::PlateController;

//#[derive(Copy, Clone)]
pub struct PhysicsController {
    start_x: f32,
    start_y: f32,
    x: f32,
    y: f32,
    speed: f32,
    max_speed: f32,
    pub acceleration: f32,
    jump_speed: f32,
    jumps_used: i8,
    last_jump_time: SystemTime,
    last_ground_time: SystemTime,
    max_jumps: i8,
    stop_speed: f32,
    fall_speed: f32,
    gravity: f32,
    max_fall_speed: f32,
    is_grounded: bool,
    can_move: bool,
    dash_time: u128,
    pre_dash_speed: f32,
    curr_direction: i8, // 1 if facing right, 0 if facing left
    colliders: Vec<RectCollider>
}

impl PhysicsController {
    pub fn new(_x: f32, _y:f32, _maxspeed: f32, _acceleration: f32, _jumpspeed:f32, _maxjumps: i8, _stopspeed: f32, _gravity: f32, _maxfallspeed: f32,  _colliders: Vec<RectCollider>)
        -> PhysicsController
    {
        PhysicsController {
            start_x: 0.0,
            start_y: 0.0,
            x: _x,
            y: _y,
            speed: 0.0,
            max_speed: _maxspeed,
            acceleration: _acceleration,
            jump_speed: _jumpspeed,
            jumps_used: _maxjumps,
            last_jump_time: SystemTime::now(),
            last_ground_time: SystemTime::now(),
            max_jumps: _maxjumps,
            stop_speed: _stopspeed,
            fall_speed: 0.0,
            gravity: _gravity,
            max_fall_speed: _maxfallspeed,
            is_grounded: false,
            can_move: true,
            dash_time: 100,
            pre_dash_speed: 0.0,
            curr_direction: 1,
            colliders: _colliders
        }
    }

    //getters
    //pub fn start_x(&self) -> f32 { self.start_x }
    //pub fn start_y(&self) -> f32 { self.start_y }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn position_rect(&self) -> (i32, i32, u32, u32) { (self.x as i32, self.y as i32, 69, 98)}
    pub fn speed(&self) -> f32 { self.speed }
    pub fn fall_speed(&self) -> f32 { self.fall_speed }
    pub fn dash_time(&self) -> u128 { self.dash_time }
    pub fn is_grounded(&self) -> bool { self.is_grounded }
    pub fn total_speed(&self) -> f32 {
        self.speed.powf(2.0) + self.fall_speed.powf(2.0).powf(0.5)
    }
    pub fn colliders(&self) -> Vec<RectCollider> {
        let mut return_vec: Vec<RectCollider> = vec!();
        for c in &self.colliders {
            return_vec.push(*c);
        }
        return_vec
    }

    //setters
    pub fn reset_jumps(&mut self) { self.jumps_used = 0; }
    pub fn immobilize(&mut self) { self.can_move = false; }
    pub fn mobilize(&mut self) { self.can_move = true; }
    pub fn set_start_x(&mut self, _x: f32) { self.start_x = _x; }
    pub fn set_start_y(&mut self, _y: f32) { self.start_y = _y; }
    pub fn set_x(&mut self, _x: f32) {self.x = _x}
    pub fn set_y(&mut self, _y: f32) {self.y = _y}
    pub fn set_speed(&mut self, _speed: f32) {self.speed = _speed}
    pub fn set_fall_speed(&mut self, _fall_speed: f32) {self.fall_speed = _fall_speed}
    pub fn set_jumps_used(&mut self, _jumps_used: i8) { self.jumps_used = _jumps_used }
    pub fn reset_colliders(&mut self) { self.colliders = vec!(); }
    pub fn respawn(&mut self) {
        self.x = self.start_x;
        self.y = self.start_y;
    }

    pub fn add_collider(&mut self, new_collider: RectCollider) {
        self.colliders.push(new_collider);
    }

    // debug: prints out a list of the controller's current state
    /*pub fn debug(&mut self) {
        println!("Physics Controller status:");
        println!("\tx: {}", self.x);
        println!("\ty: {}", self.y);
        println!("\tspeed: {}", self.speed);
        println!("\tfall speed: {}", self.fall_speed);
        println!("\tjumps used: {}/{}", self.jumps_used, self.max_jumps);
        println!("\tmoving: {}", self.is_moving());
        println!("\tgrounded: {}", self.is_grounded);
    }*/

    // accelerate_left: accelerates the character to the left
    pub fn accelerate_left(&mut self) {
        self.curr_direction = 0;
        if self.speed > -self.max_speed {
            self.speed -= self.acceleration;
        }
        if self.speed < -self.max_speed {
            //self.speed = -self.max_speed;
            self.speed += self.acceleration;
        }
    }

    // accelerate_right: accelerates the character to the right
    pub fn accelerate_right(&mut self) {
        self.curr_direction = 1;
        if self.speed < self.max_speed {
            //self.speed = self.acceleration;
            self.speed += self.acceleration;
        }
        if self.speed > self.max_speed {
            //self.speed = self.max_speed;
            self.speed -= self.acceleration;
        }
    }

    // update: manage the character's state each frame
    pub fn update(&mut self, platecon: PlateController) {
        //maybe we don't want the character to move (like finishing a level)
        if self.can_move {
            //move the character if necessary
            let mut x_valid = true;
            let mut y_valid = true;
            let my_collider_x = RectCollider::new(self.x+self.speed, self.y, 69.0, 98.0);
            let my_collider_y = RectCollider::new(self.x, self.y+self.fall_speed, 69.0, 98.0);
            self.is_grounded = false;
            for c in &self.colliders {
                if c.is_touching(&my_collider_x) {
                    x_valid = false;
                }
                if c.is_touching(&my_collider_y) {
                    y_valid = false;
                    if my_collider_y.y() < c.y() {
                        self.y = c.y() - 100.0;
                    } else {
                        self.fall_speed = 0.0;
                    }
                }
                if c.contains_point(self.x+50.0, self.y+105.0) {
                    if self.fall_speed < 0.0 {
                        y_valid = true;
                    }
                    self.is_grounded = true;
                }
            }
            // are we hitting a closed gate?
            if platecon.active_gate_collider().is_touching(&my_collider_x) {
                x_valid = false;
            }
            if platecon.active_gate_collider().is_touching(&my_collider_y) {
                y_valid = false;
                if my_collider_y.y() < platecon.active_gate_collider().y() {
                    self.y = platecon.active_gate_collider().y() - 100.0;
                } else {
                    self.fall_speed = 0.0;
                }
            }
            if platecon.active_gate_collider().contains_point(self.x+50.0, self.y+105.0) {
                if self.fall_speed < 0.0 {
                    y_valid = true;
                }
                self.is_grounded = true;
            }
            // check if x and y are valid
            if x_valid {
                self.x = (self.x + self.speed).clamp(0.0, 1211.0);  // replace 1200.0 later with (CAM_W - TILE_SIZE) vars
            }
            if y_valid {
                self.y += self.fall_speed;
            }

            // decelerate the character
            if self.speed > 0.0 {
                self.speed -= self.stop_speed;
                if self.speed < 0.0 { self.speed = 0.0; }
            } else if self.speed < 0.0 {
                self.speed += self.stop_speed;
                if self.speed > 0.0 { self.speed = 0.0; }
            }

            //simulate gravity
            if self.fall_speed < self.max_fall_speed {
                self.fall_speed += self.gravity;
            }

            if !self.is_grounded {
                self.last_ground_time = SystemTime::now();
            }

            //reset jumps if we're on the ground and we've been on the ground for a little while
            if self.is_grounded && self.fall_speed > 0.0 && self.last_ground_time+Duration::from_millis(100) < SystemTime::now() {
                self.reset_jumps();
                self.fall_speed = 0.0;
            }
        }
    }

    //jump: if we have jumps left, give ourselves a boost upwards. this is so we can support multiple jumps if we need
    pub fn jump(&mut self) {
        // if we have one jump, we have to use it on the ground
        if self.jumps_used == 0 && self.jumps_used+1 == self.max_jumps && !self.is_grounded { return; }
        // the time comparison here is to prevent jumps from occurring on successive frames, which would be frustrating to players
        if self.last_jump_time+Duration::from_millis(250) < SystemTime::now() && self.jumps_used < self.max_jumps {
            self.jumps_used += 1;
            self.fall_speed = -self.jump_speed;
            self.last_jump_time = SystemTime::now();
            self.is_grounded = false;
        }
    }

    // gives the player the ability to dash in a derection depending on the acceleration of the player
    pub fn dash(&mut self, speed: f32, first_press: i8) {
        self.gravity = 0.0;
        self.fall_speed = 0.0;
        if(first_press == 1) {
            self.pre_dash_speed = speed;
        }

        if(self.curr_direction == 1) {
            self.speed = 32.0;
        }
        else if(self.curr_direction == 0) {
            self.speed = -32.0;
        }
    }

    pub fn stop_dash(&mut self) {
        self.speed = self.pre_dash_speed;
        self.gravity = 1.0;
    }
}

impl Clone for PhysicsController {
    fn clone(&self) -> PhysicsController {
        PhysicsController {
            start_x: self.start_x,
            start_y: self.start_y,
            x: self.x,
            y: self.y,
            speed: self.speed,
            max_speed: self.max_speed,
            acceleration: self.acceleration,
            jump_speed: self.jump_speed,
            jumps_used: self.jumps_used,
            last_jump_time: self.last_jump_time,
            last_ground_time: self.last_ground_time,
            max_jumps: self.max_jumps,
            stop_speed: self.stop_speed,
            fall_speed: self.fall_speed,
            gravity: self.gravity,
            max_fall_speed: self.max_fall_speed,
            is_grounded: self.is_grounded,
            can_move: self.can_move,
            dash_time: self.dash_time,
            pre_dash_speed: self.pre_dash_speed,
            curr_direction: self.curr_direction,
            colliders: self.colliders()
        }
    }
}
