mod compute;
mod utils;
mod messages;
mod structs;
mod kalman_filter;

use std::{net::UdpSocket, time::Duration};
use structs::{Motion, Axis};
use compute::{compute_velocity, calculate_new_coordonates};
use nalgebra::Matrix6x1;
use kalman_filter::{KalmanFilter, create_filter};
use chrono::NaiveTime;
use utils::end_prog;

fn compute_message(motion: &mut Motion, first_time: &mut bool, f: &mut KalmanFilter, dt:f64, gps: &Axis) {
    if *first_time == true {
        let mut velocity = Axis{X: *motion.speed.last().unwrap(), Y: 0.0, Z: 0.0};
        compute_velocity(motion.direction.last().unwrap(), motion.acceleration.last().unwrap(), &mut velocity, dt);
        f.x = Matrix6x1::new(motion.position.last().unwrap().X, motion.position.last().unwrap().Y, motion.position.last().unwrap().Z,
                        velocity.X, velocity.Y, velocity.Z);
        *first_time = false;
    } else {
        calculate_new_coordonates(motion, f, gps);
    }
}

fn receive_messages(socket: &UdpSocket, motion: &mut Motion, first_time: &bool, gps: &mut Axis, time: &mut NaiveTime, position_debug: &mut Vec<Axis>) {
    let mut buffer = [0; 512];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((bytes_received, _)) => {
                let utf8_string = String::from_utf8_lossy(&buffer[..bytes_received]);
                if &utf8_string == "MSG_END" {
                    return;
                } else {
                    messages::dispatch_message(motion, &utf8_string, first_time, gps, time, position_debug);
                }
            }
            Err(_e) => { 
                end_prog(position_debug, &time, 1);
            }
        }
    }
}

fn main() {
    const SERVER_ADDRESSE: &str = "127.0.0.1:4242";
    let mut buffer = [0; 512];
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to address");
    let _ = socket.set_read_timeout(Some(Duration::new(1, 0)));
    let mut motion: Motion = Motion {
        position:       Vec::new(),
        speed:          Vec::new(),
        acceleration:   Vec::new(),
        direction:      Vec::new()
    };
    let mut position_debug: Vec<Axis> = Vec::new();
    let mut first_time: bool = true;
    let dt: f64 = 1.0/100.0;
    let mut filter: KalmanFilter = create_filter(dt, Matrix6x1::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    let mut time: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    messages::send_message_to(&socket, SERVER_ADDRESSE, "READY".as_bytes());
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((bytes_received, _)) => {
                let utf8_string = String::from_utf8_lossy(&buffer[..bytes_received]);
                if &utf8_string == "MSG_START" {
                    let mut gps: Axis = Axis{X:0.0, Y:0.0, Z:0.0};

                    receive_messages(&socket, &mut motion, &first_time, &mut gps, &mut time, &mut position_debug);
                    compute_message(&mut motion, &mut first_time, &mut filter, dt, &gps);
                    messages::send_message_to(&socket, SERVER_ADDRESSE, &motion.position.last().unwrap().to_string().as_bytes());
                } else if &utf8_string == "GOODBYE." {
                    end_prog(&position_debug, &time, 0);
                } else {
                    println!("message: {:?}", utf8_string);
                }
            }
            Err(_e) => {
                end_prog(&position_debug, &time, 1);
            }
        }
    }
}
