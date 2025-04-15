use std::time::{Duration, Instant};
use tokio::time::sleep;
pub mod subsystems;

//export PATH="$HOME/.gradle/toolchains/frc/2024/roborio/bin:$PATH"

use tokio::{runtime::Runtime, task::LocalSet};
use frcrs::{init_hal, hal_report, observe_user_program_starting};
use frcrs::input::{RobotState};
use crate::subsystems::Motor;

fn main() {
    println!("yipee");
    let executor = Runtime::new().unwrap();
    let local = LocalSet::new();
    executor.block_on(local.run_until(async {
        if !init_hal() {
            panic!("Failed to initalize HAL");
        }

        //hal_report(2, 7, 0, "2024.2.1".to_string());

        observe_user_program_starting();

        let mut last_loop = Instant::now();
        let motor = Motor::new();

        loop {
                //println!("we made it this far");
                let state = RobotState::get();
                let elapsed = last_loop.elapsed().as_secs_f64();
                let left = (1. / 500. - elapsed).max(0.);
                sleep(Duration::from_secs_f64(left)).await;
                //println!("{}", 1. / last_loop.elapsed().as_secs_f64());
                last_loop = Instant::now();
                println!("Enabled: {}",state.enabled());
                println!("Teleop: {}",state.teleop());
                if state.enabled() && state.teleop() {
                    println!("toast");
                    motor.set(0.05);
                }
            }
    }));

}
