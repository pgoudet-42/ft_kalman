from structs import Axis, EulerAngles
from kalman_compute import changeRefWithEuler, scalarSpeedTo3DSpeed
import numpy as np

def setValuesFirstTime(info, predict, debug):
    if info[0][14:] == "TRUE POSITION":
        predict.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
        debug.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "SPEED":
        predict.speed.append(float(info[1]))
        debug.speed.append(float(info[1]))
    elif info[0][14:] == "ACCELERATION":
        predict.acceleration.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
        debug.acceleration.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "DIRECTION":
        predict.direction.append(EulerAngles(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
        debug.direction.append(EulerAngles(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    return info[0][14:]

def init_speed3d(predict):
    vec = np.array([1,1,1])
    vector = changeRefWithEuler(vec, predict.direction[-1].ψ, predict.direction[-1].θ, predict.direction[-1].φ)
    norm = np.linalg.norm(vector)
    vector_normalise = vector / norm
    vector_normalise = Axis(X=vector_normalise[0], Y=vector_normalise[1], Z=vector_normalise[2])
    speed3D = scalarSpeedTo3DSpeed(vector_normalise, np.float64(predict.speed[-1]))
    predict.speed3d.append(Axis(X=speed3D.X, Y=speed3D.Y, Z=speed3D.Z)) 