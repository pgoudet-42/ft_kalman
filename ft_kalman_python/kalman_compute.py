import numpy as np
from structs import Axis, Motion, EulerAngles

def scalarSpeedTo3DSpeed(vector: Axis, speed):
    speed3D: Axis = Axis(
        X = speed * vector.X,
        Y = speed * vector.Y,
        Z = speed * vector.Z
    )
    return speed3D

def speed3DToScalar(speed3d: Axis):
    return np.sqrt(speed3d.X ** 2 + speed3d.Y ** 2 + speed3d.Z ** 2)

def changeRefWithEuler(vec, ψ, θ, φ):
    A = np.array([
        [ np.cos(ψ)*np.cos(φ) - np.sin(ψ)*np.cos(θ)*np.sin(φ), -np.cos(ψ)*np.cos(φ) - np.sin(ψ)*np.cos(θ)*np.sin(φ), np.sin(ψ)*np.sin(θ)  ],
        [ np.sin(ψ)*np.cos(φ) + np.cos(ψ)*np.cos(θ)*np.sin(φ), -np.sin(ψ)*np.sin(φ) + np.cos(ψ)*np.cos(θ)*np.cos(φ), -np.cos(ψ)*np.sin(θ) ],
        [ np.sin(θ)*np.sin(φ)                                   , np.sin(θ)*np.cos(φ)                                 , np.cos(θ)         ],
    ])
    return np.dot(A.T, vec)

def computeNewPosition(predict: Motion):
    time = 1/300
    acceleration = np.array([predict.acceleration[-1].X, predict.acceleration[-1].Y, predict.acceleration[-1].Z])
    old_position = np.array([predict.position[-1].X, predict.position[-1].Y, predict.position[-1].Z])
    speed3d = np.array([predict.speed3d[-1].X, predict.speed3d[-1].Y, predict.speed3d[-1].Z])
    new_position = old_position + speed3d * time + 1/2 * acceleration * time ** 2
    return Axis(X=new_position[0], Y=new_position[1], Z=new_position[2])

def calculateNewCoordonates(predict: Motion, new_acceleration: Axis, new_orientation: EulerAngles):
    v = np.array([predict.direction[-1].ψ, predict.direction[-1].θ, predict.direction[-1].φ])
    vector = changeRefWithEuler(v, new_orientation.ψ, new_orientation.θ, new_orientation.φ)
    norm = np.linalg.norm(vector)
    vector_normalise = vector / norm
    vector_normalise = Axis(X=vector_normalise[0], Y=vector_normalise[1], Z=vector_normalise[2])
    speed3D = scalarSpeedTo3DSpeed(vector_normalise, np.float64(predict.speed[-1]))
    predict.speed3d.append(speed3D)
    # predict.speed3d.append(Axis(
    #     X=speed3D.X + new_acceleration.X,
    #     Y=speed3D.Y + new_acceleration.Y,
    #     Z=speed3D.Z + new_acceleration.Z,
    # ))
    predict.speed.append(speed3DToScalar(predict.speed3d[-1]))
    predict.acceleration.append(new_acceleration)
    predict.direction.append(new_orientation)
    predict.position.append(computeNewPosition(predict))
    # print("verification:", np.sqrt(predict.speed3d[-1].X ** 2 + predict.speed3d[-1].Y ** 2 + predict.speed3d[-1].Z ** 2))
    # print(predict)
    # exit(1)