#![allow(non_snake_case)]
use nalgebra::{Matrix3, Matrix3x1, Matrix3x6, Matrix6, Matrix6x1, Matrix6x3};


/// KlamanFilter struct
/// dt is the delta time between each sensor data received
/// x is the state vector
/// F is the Transition matrix
/// B is the control matrix
/// H is the measurement matrix
/// Q is the process noise matrix
/// R is the measurement uncertainty
/// P is the covariance matrix
pub struct KalmanFilter {
        y: Matrix3x1<f64>,
    pub x: Matrix6x1<f64>,
    pub F: Matrix6<f64>, 
    pub B: Matrix6x3<f64>,
    pub H: Matrix3x6<f64>,
    pub Q: Matrix6<f64>,
    pub R: Matrix3<f64>,
    pub P: Matrix6<f64>,
        S: Matrix3<f64>,
        K: Matrix6x3<f64>,
        I: Matrix6<f64>
}

impl KalmanFilter {
    pub fn predict(&mut self, u: &Matrix3x1<f64>) {
        self.x = self.F * self.x + self.B * u;
        self.P = self.F * self.P * self.F.transpose() + self.Q;
    }

    pub fn update(&mut self, z: &Matrix3x1<f64>) {
        self.y = z - self.H * self.x;
        // # S = HPH' + R
        // # project system uncertainty into measurement space
        self.S = self.H * self.P * self.H.transpose() + self.R;
        // map system uncertainty into kalman gain
        self.K = self.P * self.H.transpose() * self.S.try_inverse().unwrap();
        // # x = x + Ky
        // # predict new x with residual scaled by the kalman gain
        self.x = self.x + self.K * self.y;
        // # P = (I-KH)P(I-KH)' + KRK'
        // # This is more numerically stable
        // # and works for non-optimal K vs the equation
        // # P = (I-KH)P usually seen in the literature.
        let i_kh = self.I - self.K * self.H;
        self.P = i_kh * self.P * i_kh.transpose() + self.K * self.R * self.K.transpose();
    }
}

pub fn create_filter(dt: f64, init_state: Matrix6x1<f64>) -> KalmanFilter {
    let σ_gps = 10_f64.powi(-1);
    let σ_acc = 10_f64.powi(-3);
    let σ_gyr = 10_f64.powi(-2);

    let F: Matrix6<f64> = Matrix6::new(
        1.0, 0.0, 0.0, dt, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0 ,dt, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0, dt,
        0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let H: Matrix3x6<f64> = Matrix3x6::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0 ,0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0
    );

    let B: Matrix6x3<f64> = Matrix6x3::new(
        dt.powi(2) / 2.0, 0.0             , 0.0             ,
        0.0             , dt.powi(2) / 2.0, 0.0             ,
        0.0             , 0.0             , dt.powi(2) / 2.0,
        dt              , 0.0             , 0.0             ,
        0.0             , dt              , 0.0             ,
        0.0             , 0.0             , dt              
    );

    let continous_position: Matrix6<f64> = Matrix6::new(
        dt.powi(2) / 2.0, 0.0              , 0.0               , 0.0, 0.0, 0.0,
        0.0             , dt.powi(2) / 2.0 , 0.0               , 0.0, 0.0, 0.0,
        0.0             , 0.0              , dt.powi(2) / 2.0  , 0.0, 0.0, 0.0,
        0.0             , 0.0              , 0.0               , 0.0, 0.0, 0.0,
        0.0             , 0.0              , 0.0               , 0.0, 0.0, 0.0,
        0.0             , 0.0              , 0.0               , 0.0, 0.0, 0.0
    );

    let continous_acceleration: Matrix6<f64> = Matrix6::new(
        dt.powi(2) / 2.0, 0.0              , 0.0               , dt.powi(2) / 2.0  , 0.0               , 0.0               ,
        0.0             , dt.powi(2) / 2.0 , 0.0               , 0.0               , dt.powi(2) / 2.0  , 0.0               ,
        0.0             , 0.0              , dt.powi(2) / 2.0  , 0.0               , 0.0               , dt.powi(2) / 2.0  ,
        0.0             , 0.0              , 0.0               , dt                , 0.0               , 0.0               ,
        0.0             , 0.0              , 0.0               , 0.0               , dt                , 0.0               ,
        0.0             , 0.0              , 0.0               , 0.0               , 0.0               , dt                
    );

    let continous_speed: Matrix6<f64> = Matrix6::new(
        0.0, 0.0, 0.0, dt.powi(2) / 2.0, 0.0               , 0.0               ,
        0.0, 0.0, 0.0, 0.0             , dt.powi(2) / 2.0  , 0.0               ,
        0.0, 0.0, 0.0, 0.0             , 0.0               , dt.powi(2) / 2.0  ,
        0.0, 0.0, 0.0, dt              , 0.0               , 0.0               ,
        0.0, 0.0, 0.0, 0.0             , dt                , 0.0               ,
        0.0, 0.0, 0.0, 0.0             , 0.0               , dt                
    );

    let var_p = σ_gps.powi(2);
    let var_s = σ_gyr.powi(2) + σ_acc.powi(2) * dt;
    let var_a = σ_acc.powi(2);

    let noise_position = Matrix6::new(
        var_p, 0.0  , 0.0  , 0.0, 0.0, 0.0,
        0.0  , var_p, 0.0  , 0.0, 0.0, 0.0,
        0.0  , 0.0  , var_p, 0.0, 0.0, 0.0,
        0.0  , 0.0  , 0.0  , 0.0, 0.0, 0.0,
        0.0  , 0.0  , 0.0  , 0.0, 0.0, 0.0,
        0.0  , 0.0  , 0.0  , 0.0, 0.0, 0.0
    );

    let noise_acceleration = Matrix6::new(
        dt.powi(2) / 2.0 * var_a, 0.0                     , 0.0                     , dt.powi(2) / 2.0 * var_a, 0.0                     , 0.0                     ,
        0.0                     , dt.powi(2) / 2.0 * var_a, 0.0                     , 0.0                     , dt.powi(2) / 2.0 * var_a, 0.0                     ,
        0.0                     , 0.0                     , dt.powi(2) / 2.0 * var_a, 0.0                     , 0.0                     , dt.powi(2) / 2.0 * var_a,
        0.0                     , 0.0                     , 0.0                     , dt * var_a              , 0.0                     , 0.0                     ,
        0.0                     , 0.0                     , 0.0                     , 0.0                     , dt * var_a              , 0.0                     ,
        0.0                     , 0.0                     , 0.0                     , 0.0                     , 0.0                     , dt * var_a              
    );
    
    let noise_speed = Matrix6::new(
        0.0, 0.0, 0.0, dt * var_s, 0.0       , 0.0       ,
        0.0, 0.0, 0.0, 0.0       , dt * var_s, 0.0       ,
        0.0, 0.0, 0.0, 0.0       , 0.0       , dt * var_s,
        0.0, 0.0, 0.0, var_s     , 0.0       , 0.0       ,
        0.0, 0.0, 0.0, 0.0       ,var_s      , 0.0       ,
        0.0, 0.0, 0.0, 0.0       , 0.0       ,var_s      
    );

    let noise: Matrix6<f64> = continous_position * noise_position + continous_acceleration * noise_acceleration + continous_speed * noise_speed;
    let mut Q: Matrix6<f64> = F * noise* F.transpose();
    // integrer le tout
    Q *= dt;

    let R: Matrix3<f64> = Matrix3::new(
        σ_gps.powi(2)  , 0.0           , 0.0           ,
        0.0            , σ_gps.powi(2) , 0.0           ,
        0.0            , 0.0           , σ_gps.powi(2) 
    );

    let P: Matrix6<f64> = Matrix6::new(
        σ_gps.powi(2)  , 0.0           , 0.0           , 0.0                           , 0.0                           , 0.0                           ,
        0.0            , σ_gps.powi(2) , 0.0           , 0.0                           , 0.0                           , 0.0                           ,
        0.0            , 0.0           , σ_gps.powi(2) , 0.0                           , 0.0                           , 0.0                           ,
        0.0            , 0.0           , 0.0           , σ_acc.powi(2) + σ_gyr.powi(2) , 0.0                           , 0.0                           ,
        0.0            , 0.0           , 0.0           , 0.0                           , σ_acc.powi(2) + σ_gyr.powi(2) , 0.0                           ,
        0.0            , 0.0           , 0.0           , 0.0                           , 0.0                           , σ_acc.powi(2) + σ_gyr.powi(2) 
    );

    return KalmanFilter{y: Matrix3x1::zeros(), x: init_state, F: F, B: B, H: H, Q: Q, R: R, P: P, S: Matrix3::zeros(), K: Matrix6x3::zeros(), I: Matrix6::identity()};
}
