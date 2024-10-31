import numpy as np
from structs import Axis, Motion, EulerAngles
from filterpy.kalman import KalmanFilter
from datetime import datetime

def rotationMatrix(direction):
    ψ, θ, φ = direction.ψ, direction.θ, direction.φ

    R_z = np.array([
        [np.cos(ψ), -np.sin(ψ), 0],
        [np.sin(ψ), np.cos(ψ), 0],
        [0, 0, 1]
    ])

    R_y = np.array([
        [np.cos(θ), 0, np.sin(θ)],
        [0, 1, 0],
        [-np.sin(θ), 0, np.cos(θ)]
    ])

    R_x = np.array([
        [1, 0, 0],
        [0, np.cos(φ), -np.sin(φ)],
        [0, np.sin(φ), np.cos(φ)]
    ])

    R = np.dot(R_z, np.dot(R_y, R_x))
    return R

def computeVelocity(direction, acceleration, velocity, dt):
    R = rotationMatrix(direction)

    acc = np.dot(R, acceleration.to_nparray())
    velocity[0] += acc[0] * dt
    velocity[1] += acc[1] * dt
    velocity[2] += acc[2] * dt
    
    return velocity, acc

def calculateNewCoordonates(predict: Motion, new_acceleration: Axis, new_orientation: EulerAngles, f: KalmanFilter, args, gps: Axis=None, debug: Motion=None, time: datetime=None):

    predict.acceleration.append(new_acceleration)
    predict.direction.append(new_orientation)
    ψ, θ, φ = predict.direction[-1].ψ, predict.direction[-1].θ, predict.direction[-1].φ

    acc_global = predict.acceleration[-1].to_nparray()

    f.predict(u=acc_global)
    predict.position.append(Axis(float(f.x[0][0]), float(f.x[1][0]), float(f.x[2][0])))
    predict.speed.append(np.linalg.norm(np.array([f.x[3][0], f.x[4][0], f.x[5][0]])))
    predict.speed3d.append(Axis(float(f.x[3][0]), float(f.x[4][0]), float(f.x[5][0])))

    if gps.is_notinitialized() == False:
        f.update(gps.to_nparray())
    elif args.debug:
        if time.second > 0 and time.microsecond == 0 and time.second % 3 == 0:
            f.update(debug.position[-1].to_nparray())
        
    

