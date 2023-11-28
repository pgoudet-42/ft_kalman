#[allow(non_snake_case)]
pub struct Axis {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}


#[allow(dead_code)]
pub struct Motion {
    pub location: Axis,
    pub speed: f64,
    pub acceleration: Axis,
    pub direction: Axis,
    pub gps_location: Axis,
}

pub fn print_motion(motion: &mut Motion) {
    println!("location X: {} Y: {} Z: {}", motion.location.X, motion.location.Y, motion.location.Z);
    println!("speed: {}", motion.speed);
    println!("acceleration: X: {} Y: {} Z: {}", motion.acceleration.X, motion.acceleration.Y, motion.acceleration.Z);
    println!("direction X: {} Y: {} Z: {}", motion.direction.X, motion.direction.Y, motion.direction.Z);
    println!("gps location X: {} Y: {} Z: {}", motion.gps_location.X, motion.gps_location.Y, motion.gps_location.Z);
}

pub fn set_speed(values: &Vec<&str>, speed: &mut f64) {
    match values[1].parse::<f64>() {
        Ok(s) => *speed = s,
        Err(e) => println!("Error in speed field conversion {e}")
    }
}

pub fn set_axis(values: &Vec<&str>, axis: &mut Axis, field: &str) {
    match values[1].parse::<f64>() {
        Ok(x)=> axis.X = x,
        Err(e) => println!("Error in {field} fields conversion {e}")
    }
    match values[2].parse::<f64>() {
        Ok(y)=> axis.Y = y,
        Err(e) => println!("Error in {field} fields conversion {e}")
    }
    match values[3].parse::<f64>() {
        Ok(z)=> axis.Z = z,
        Err(e) => println!("Error in {field} fields conversion {e}")
    }
}