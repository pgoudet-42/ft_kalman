use nalgebra::{Matrix3, Matrix3x1, Vector3};
use crate::{kalman_filter::KalmanFilter, structs::{Motion, Axis, EulerAngles}};


fn rotation_matrix(direction: &EulerAngles) -> Matrix3<f64> {
    let (ψ, θ, φ) = (direction.ψ, direction.θ, direction.φ);

    let r_z = Matrix3::new(
                        ψ.cos(), -ψ.sin(), 0.0, 
                        ψ.sin(), ψ.cos() , 0.0,
                        0.0    , 0.0     , 1.0
                    );
    let r_y = Matrix3::new(   
                        θ.cos() , 0.0, θ.sin(), 
                        0.0     , 1.0, 0.0    ,
                        -θ.sin(), 0.0, θ.cos()
                    );

    let r_x = Matrix3::new(   
                        1.0, 0.0    , 0.0     , 
                        0.0, φ.cos(), -φ.sin(),
                        0.0, φ.sin(), φ.cos() 
                    );
    
    let r = r_z* r_y * r_x;

    return r
}

pub fn compute_velocity(direction: &EulerAngles, acceleration: &Axis, velocity: &mut Axis, dt: f64) {
    let r: Matrix3<f64> = rotation_matrix(direction);

    let acc = r * acceleration.to_vec();
    velocity.X += acc[0] * dt;
    velocity.Y += acc[1] * dt;
    velocity.Z += acc[2] * dt;
}

pub fn calculate_new_coordonates(motion: &mut Motion, f: &mut KalmanFilter, gps: &Axis) {
    let acc_global: Matrix3x1<f64> = motion.acceleration.last().unwrap().to_vec();
    let u: Matrix3x1<f64> = Matrix3x1::new(acc_global[0], acc_global[1], acc_global[2]);

    f.predict(&u);
    motion.position.push(Axis{X: f.x[0], Y: f.x[1], Z: f.x[2]});
    motion.speed.push(Vector3::new(f.x[0], f.x[1],f.x[2]).norm());

    if gps.is_init() == true {
        f.update(&gps.to_vec())
    }

}