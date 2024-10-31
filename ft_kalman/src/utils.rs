use chrono::NaiveTime;
use crate::structs::Axis;
// use plotters::prelude::*;
// mod structs;
use crate::structs::Motion;

#[allow(dead_code)]
pub fn print_motion(motion: &mut Motion) {
    println!("position X: {} Y: {} Z: {}", motion.position.last().unwrap().X, motion.position.last().unwrap().Y, motion.position.last().unwrap().Z);
    println!("speed: {}", motion.speed.last().unwrap());
    println!("acceleration: X: {} Y: {} Z: {}", motion.acceleration.last().unwrap().X, motion.acceleration.last().unwrap().Y, motion.acceleration.last().unwrap().Z);
    println!("direction φ: {} θ: {} ψ: {}", motion.direction.last().unwrap().φ, motion.direction.last().unwrap().θ, motion.direction.last().unwrap().ψ);
}


pub fn end_prog(_position_debug: &Vec<Axis>, time: &NaiveTime, err_code: i32) {
    println!("Time: {}", time);
    std::process::exit(err_code);
}