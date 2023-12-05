// use graplot::Plot3D;
use crate::utils::Motion;

// fn display_positions() {
//     println!("{:?}", POSITIONS.lock().unwrap().get(0).unwrap().X);
    
// }


// pub fn display_graph()-> Result<(), Box<dyn std::error::Error>> {
   
// }


pub fn create_buffer(motion: &mut Motion) -> String {
    let buffer = motion.location.X.to_string() + " " + &motion.location.Y.to_string() + " " + &motion.location.Z.to_string();
    return buffer;
}


