use std::time::{Duration, Instant};
use tokio::time::sleep;
pub mod subsystems;

pub mod constants;

use frcrs::input::{Joystick, RobotState};
use frcrs::{hal_report, init_hal, observe_user_program_starting, refresh_data};
use motormove::{teleop, Ferris, Joysticks};
use tokio::{runtime::Runtime, task::LocalSet};

fn main() {
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();
    executor.block_on(local.run_until(async {
        if !init_hal() {
            panic!("Failed to initalize HAL");
        }

        observe_user_program_starting();

        let mut last_loop = Instant::now();
        let mut ferris = Ferris::new();
        let mut joysticks = Joysticks::new();

        loop {
            refresh_data();
            let state = RobotState::get();
            let elapsed = last_loop.elapsed().as_secs_f64();
            let left = (1. / 500. - elapsed).max(0.);
            sleep(Duration::from_secs_f64(left)).await;
            //println!("{}", 1. / last_loop.elapsed().as_secs_f64());
            last_loop = Instant::now();
            if state.enabled() && state.teleop() {
                teleop(&mut ferris, &mut joysticks)
            }
        }
    }));
}
