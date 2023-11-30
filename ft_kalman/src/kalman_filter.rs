mod utils;
use utils::{Axis, Motion, Vector};
use numpy::ndarray::Array2;
use numpy::ndarray::Array1;

// fn state_equ(motion: &mut Motion, known_entries: Motion, random_signals: Motion ) {
//     motion = a * motion + b * known_entries + m * random_signals;
// }

// fn mesure_equ(motion: &mut Motion, known_entries: Motion, mesure_vector: Motion ) {
//     motion = c * motion + d * known_entries + mesure_vector;
// }

struct Kalman_Filter {
    dt: float64,
    E: Array1,
    A: Array2,
    H: Array2,
    Q: Array2,
    R: Array2,
    P: Array2,
}

impl Kalman_Filter {
    fn get_vectors_init_state(motion: &mut Motion) {
        vec = Array1::new([
            motion.location.X,
            motion.location.Y,
            motion.location.Z,
            motion.acceleration.X,
            motion.acceleration.Y,
            motion.acceleration.Z,
        ]);
        return (vec);
    }

    fn get_observation_matrice(motion: &mut Motion) {
        let matrice = Array2::new([
            [1.0, 0, 0, 0, 0, 0],
            [0, 1.0, 0, 0, 0, 0],
            [0, 0, 1.0, 0, 0, 0],
        ]);
        return (matrice);
    }

    fn get_transition_matrice(motion: &mut Motion, dt: float) {
        let matrice = Array2::new([
            [1.0, 0, 0, dt, 0, 0],
            [0, 1.0, 0, 0 ,dt, 0],
            [0, 0, 1.0, 0, 0, dt],
            [0, 0, 0, 1.0, 0, 0],
            [0, 0, 0, 0, 1.0, 0],
            [0, 0, 0, 0, 0, 1.0],
        ]);
        return (matrice);
    }

    fn get_noise_state_matrice() {
        let matrice = Array2::new([
            [1.0, 0, 0, 0, 0, 0],
            [0, 1.0, 0, 0, 0, 0],
            [0, 0, 1.0, 0, 0, 0],
            [0, 0, 0, 1.0, 0, 0],
            [0, 0, 0, 0, 1.0, 0],
            [0, 0, 0, 0, 0, 1.0],
        ]);
        return (matrice);
    }

    fn get_noise_mesures_matrice() {
        let matrice = Array2::new([
            [1.0, 0, 0],
            [0, 1.0, 0],
            [0, 0, 1.0],
        ]);
        return (matrice);
    }

    fn predict_state(motion: &mut Motion) {
        self.E =  self.A.dot(self.E);
        // calcul covariance:
        self.P = (self.A.dot(self.P)).dot(self.A.T) + self.Q
        return self.E
    }

    fn update_state(motion: &mut Motion) {
        let S = self.H.dot(self.P.dot(self.H.T)) + self.R;
        let K = self.P.dot(self.H.T).dot(S.inv());
    }
}
def update(self, z):


        # Calcul du gain de Kalman
        S=np.dot(self.H, np.dot(self.P, self.H.T))+self.R
        K=np.dot(np.dot(self.P, self.H.T), np.linalg.inv(S))

        # Correction / innovation
        self.E=np.round(self.E+np.dot(K, (z-np.dot(self.H, self.E))))
        I=np.eye(self.H.shape[1])
        self.P=(I-(K*self.H))*self.P

        return self.E