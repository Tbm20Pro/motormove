pub mod subsystems;
pub mod constants;

use frcrs::input::Joystick;
use crate::constants::joystick_map::JOYSTICK_ID;
use crate::subsystems::Motor;

pub struct Ferris {
    motor: Motor
}
impl Ferris {
    pub fn new() -> Ferris {
        Ferris {
            motor: Motor::new(),
        }
    }
}
pub struct Joysticks {
    stick: Joystick
}

impl Joysticks {
    pub fn new() -> Joysticks {
        Joysticks {
            stick: Joystick::new(JOYSTICK_ID),
        }
    }
}


pub fn teleop(ferris: &mut Ferris, sticks: &mut Joysticks) {
    // println!("{}", sticks.stick.get_x());
    ferris.motor.set(sticks.stick.get_x());
    ferris.motor.set(sticks.stick.get_y());
}