from structs import Axis, EulerAngles
from display_graphs import displayGraph2Dxy, displayGraph2Dyz, displayGraph2Dxz, displayGraph3D, displayGraphSpeed, displayVariances

def setValuesOtherTimes(info, debug, acceleration, direction, gps):
    if info[0][14:] == "TRUE POSITION":
        debug.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    if info[0][14:] == "POSITION":
        gps.X, gps.Y, gps.Z = (float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2]))
        debug.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "SPEED":
        debug.speed.append(float(info[1]) * (5/18)) # passer de km/h a m/s
    elif info[0][14:] == "ACCELERATION":
        acceleration.X, acceleration.Y, acceleration.Z = (float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2]))
        debug.acceleration.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "DIRECTION":
        direction.φ, direction.θ, direction.ψ = (float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2]))
        debug.direction.append(EulerAngles(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))

def setValuesFirstTime(info, predict, debug):
    if info[0][14:] == "TRUE POSITION":
        predict.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
        debug.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "SPEED":
        predict.speed.append(float(info[1]) / 3.6)
        debug.speed.append(float(info[1]) / 3.6)
    elif info[0][14:] == "ACCELERATION":
        predict.acceleration.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
        debug.acceleration.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "DIRECTION":
        predict.direction.append(EulerAngles(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
        debug.direction.append(EulerAngles(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))

def endProg(udp_socket, predict, debug, variances, e, time):
    print("Time: ", time)
    udp_socket.close()
    displayGraph3D(predict, debug)
    displayGraph2Dxy(predict, debug)
    displayGraph2Dyz(predict, debug)
    displayGraph2Dxz(predict, debug)
    displayGraphSpeed(predict, debug)
    displayVariances(variances)
    exit(0)