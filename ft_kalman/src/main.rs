use std::net::UdpSocket;
mod utils;
mod messages;
use utils::{Motion, Axis};



fn dispatch_message(motion: &mut Motion, message: &str) {
    const FIELDS: [&str; 5] = ["TRUE POSITION", "ACCELERATION", "DIRECTION", "GPS", "SPEED"];
    let axis: [&mut Axis; 4] = [&mut motion.location, &mut motion.acceleration, &mut motion.direction, &mut motion.gps_location];
    let values: Vec<&str> = message.split('\n').collect();

    if let Some(index) = FIELDS.iter().position(|&x| x == &values[0][14..]) {
        if index == 4 {
            utils::set_speed(&values, &mut motion.speed);
        } else {
            utils::set_axis(&values, axis[index], FIELDS[index]);
        }
    } else {
        println!("The field '{}' hasn't been found", values[0]);
    }
}

fn receive_messages(socket: &UdpSocket, motion: &mut Motion) {
    let mut buffer = [0; 512];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((bytes_received, _)) => {
                let utf8_string = String::from_utf8_lossy(&buffer[..bytes_received]);
                println!("message received: {:?}", &utf8_string);
                if &utf8_string == "MSG_END" {
                    return
                } else {
                    dispatch_message(motion, &utf8_string);
                }
            }
            Err(e) => eprintln!("Error receiving data: {}", e)
        }
    }
}

// fn main() {
//     const SERVER_ADDRESSE: &str = "127.0.0.1:4242";
//     let mut buffer = [0; 512];
//     let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to address");
//     let mut motion: Motion = Motion {
//         location: Axis {X: 0.0, Y: 0.0, Z: 0.0},
//         speed: 0.0,
//         acceleration: Axis {X: 0.0, Y: 0.0, Z: 0.0},
//         direction: Axis {X: 0.0, Y: 0.0, Z: 0.0},
//         gps_location: Axis {X: 0.0, Y: 0.0, Z: 0.0}
//     };

//     utils::send_message_to(&socket, SERVER_ADDRESSE, "READY".as_bytes());

//     loop {
//         match socket.recv_from(&mut buffer) {
//             Ok((bytes_received, _)) => {
//                 let utf8_string = String::from_utf8_lossy(&buffer[..bytes_received]);
//                 if &utf8_string == "MSG_START" { 
//                     receive_messages(&socket, &mut motion);
//                     messages::display_graph();
//                     // println!("response: {:?}", messages::create_buffer(&mut motion));
//                     utils::send_message_to(&socket, SERVER_ADDRESSE, messages::create_buffer(&mut motion).as_bytes());
//                 } else {
//                     println!("message different of start: {:?}", utf8_string);
//                 }
//             }
//             Err(e) => eprintln!("Error receiving data: {}", e)
//         }
//     }
// }

use gnuplot::Figure;

fn main() {
    // Données pour le graphe 3D
    let x = vec![0.0, 1.0, 2.0, 3.0];
    let y = vec![0.0, 1.0, 2.0, 3.0];
    let z = vec![0.0, 1.0, 4.0, 9.0];

    // Création de la figure 3D
    let mut fg = Figure::new();
    {
        let ax = fg.axes3d();

        // Tracé du graphe 3D
        ax.points(&x, &y, &z, &[gnuplot::Caption("Points")]);
    }

    // Affichage de la figure
    fg.show().unwrap();
}
