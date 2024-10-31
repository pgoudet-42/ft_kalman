use std::net::UdpSocket;
use chrono::NaiveTime;

use crate::structs::Motion;
use crate::structs::EulerAngles;
use crate::structs::Axis;
use crate::utils::end_prog;


fn parse_speed(values: &Vec<&str>, speed: &mut Vec<f64>) {
    match values[1].parse::<f64>() {
        Ok(s) => speed.push(s / 3.6),
        Err(e) => println!("Error in speed field conversion {e}")
    }
}

fn parse_axis(values: &Vec<&str>) -> Axis {
    let mut local_axis: Axis = Axis{X: 0.0, Y: 0.0, Z: 0.0};
    match values[1].parse::<f64>() {
        Ok(x)=> local_axis.X = x,
        Err(e) => println!("Error in POSITION OR ACCELERATION fields conversion {e}")
    }
    match values[2].parse::<f64>() {
        Ok(y)=> local_axis.Y = y,
        Err(e) => println!("Error in POSITION OR ACCELERATION fields conversion {e}")
    }
    match values[3].parse::<f64>() {
        Ok(z)=> local_axis.Z = z,
        Err(e) => println!("Error in POSITION OR ACCELERATION fields conversion {e}")
    }
    return local_axis;
}

fn parse_direction(values: &Vec<&str>) -> EulerAngles {
    let mut local_direction: EulerAngles = EulerAngles{φ: 0.0, θ: 0.0, ψ: 0.0};
    match values[1].parse::<f64>() {
        Ok(φ)=> local_direction.φ = φ,
        Err(e) => println!("Error in DIRECTION fields conversion {e}")
    }
    match values[2].parse::<f64>() {
        Ok(θ)=> local_direction.θ = θ,
        Err(e) => println!("Error in DIRECTION fields conversion {e}")
    }
    match values[3].parse::<f64>() {
        Ok(ψ)=> local_direction.ψ = ψ,
        Err(e) => println!("Error in DIRECTION fields conversion {e}")
    }
    return local_direction;
}

pub fn dispatch_message(motion: &mut Motion, message: &str, first_time: &bool, gps: &mut Axis, time: &mut NaiveTime, position_debug: &mut Vec<Axis>) {
    let values: Vec<&str> = message.split('\n').collect();
    match NaiveTime::parse_from_str(&values[0][0..14], "[%H:%M:%S%.f]") {
        Ok(naive_time) => { *time = naive_time; }
        Err(_e) => { 
            end_prog(position_debug, time, 1); 
        }
    }
    if &values[0][14..] == "SPEED" {
        parse_speed(&values, &mut motion.speed);
    } else if &values[0][14..] == "DIRECTION" {
        motion.direction.push(parse_direction(&values));
    } else if (&values[0][14..] == "POSITION" ||  &values[0][14..] == "TRUE POSITION") && *first_time == true {
        motion.position.push(parse_axis(&values));
        position_debug.push(parse_axis(&values));
    } else if (&values[0][14..] == "POSITION" ||  &values[0][14..] == "TRUE POSITION") && *first_time == false {
        *gps = parse_axis(&values);
    } else if &values[0][14..] == "ACCELERATION" {
        motion.acceleration.push(parse_axis(&values));
    } else {
        println!("The field '{}' hasn't been found", values[0]);
    }
}

pub fn send_message_to(socket: &UdpSocket, addr: &str, buffer: &[u8]) {
    match socket.send_to(buffer, addr) {
        Ok(_bytes_sent) => {},
        Err(e) =>eprintln!("Error sending data: {}", e)
    }
}
