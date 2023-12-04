// mod utils;
// use utils::{Axis, Motion, Vector};
// use numpy::ndarray::Array2;
// use numpy::ndarray::Array1;

// // fn state_equ(motion: &mut Motion, known_entries: Motion, random_signals: Motion ) {
// //     motion = a * motion + b * known_entries + m * random_signals;
// // }

// // fn mesure_equ(motion: &mut Motion, known_entries: Motion, mesure_vector: Motion ) {
// //     motion = c * motion + d * known_entries + mesure_vector;
// // }

// struct Kalman_Filter {
//     dt: float64,
//     E: Array1,
//     A: Array2,
//     H: Array2,
//     Q: Array2,
//     R: Array2,
//     P: Array2,
// }

// impl Kalman_Filter {
//     fn get_vectors_init_state(motion: &mut Motion) {
//         vec = Array1::new([
//             motion.location.X,
//             motion.location.Y,
//             motion.location.Z,
//             motion.acceleration.X,
//             motion.acceleration.Y,
//             motion.acceleration.Z,
//         ]);
//         return (vec);
//     }

//     fn get_observation_matrice(motion: &mut Motion) {
//         let matrice = Array2::new([
//             [1.0, 0, 0, 0, 0, 0],
//             [0, 1.0, 0, 0, 0, 0],
//             [0, 0, 1.0, 0, 0, 0],
//         ]);
//         return (matrice);
//     }

//     fn get_transition_matrice(motion: &mut Motion, dt: float) {
//         let matrice = Array2::new([
//             [1.0, 0, 0, dt, 0, 0],
//             [0, 1.0, 0, 0 ,dt, 0],
//             [0, 0, 1.0, 0, 0, dt],
//             [0, 0, 0, 1.0, 0, 0],
//             [0, 0, 0, 0, 1.0, 0],
//             [0, 0, 0, 0, 0, 1.0],
//         ]);
//         return (matrice);
//     }

//     fn get_noise_state_matrice() {
//         let matrice = Array2::new([
//             [1.0, 0, 0, 0, 0, 0],
//             [0, 1.0, 0, 0, 0, 0],
//             [0, 0, 1.0, 0, 0, 0],
//             [0, 0, 0, 1.0, 0, 0],
//             [0, 0, 0, 0, 1.0, 0],
//             [0, 0, 0, 0, 0, 1.0],
//         ]);
//         return (matrice);
//     }

//     fn get_noise_mesures_matrice() {
//         let matrice = Array2::new([
//             [1.0, 0, 0],
//             [0, 1.0, 0],
//             [0, 0, 1.0],
//         ]);
//         return (matrice);
//     }

//     fn predict_state(motion: &mut Motion, dt: int) {
//         self.A = get_transition_matrice(dt);
//         self.E =  self.A.dot(self.E); // location predition

//         self.P = (self.A.dot(self.P)).dot(self.A.T) + self.Q;
//         return self.E
//     }

//     fn update_state(motion: &mut Motion) {
//         ...
//     }
// }
