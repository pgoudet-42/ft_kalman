#!/usr/bin/python3

import socket
import matplotlib.pyplot as plt
from structs import Axis, Motion, EulerAngles
from kalman_compute import calculateNewCoordonates
from init_values import init_speed3d, setValuesFirstTime

def setValuesOtherTimes(info, debug, acceleration, orientation):
    if info[0][14:] == "TRUE POSITION":
        debug.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "SPEED":
        debug.speed.append(float(info[1]))
    elif info[0][14:] == "ACCELERATION":
        acceleration.X, acceleration.Y, acceleration.Z = (float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2]))
        debug.acceleration.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "DIRECTION":
        orientation.ψ, orientation.θ, orientation.φ = (float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2]))
        debug.direction.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    return info[0][14:]


def getData(udp_socket, predict, debug, first_time):
    newValues_acceleration: Axis = Axis(X=0, Y=0, Z=0)
    newValues_orientation: EulerAngles = EulerAngles(ψ=0, θ=0, φ=0)
    data = b""
    while data != "MSG_END".encode():
        try:
            data, _ = udp_socket.recvfrom(1024)
        except TimeoutError as e:
            print(e.__repr__())
            exit(1)
        mess = data.decode()
        info = mess.split("\n")
        print(data)
        setValuesFirstTime(info, predict, debug) if first_time else setValuesOtherTimes(info, debug, newValues_acceleration, newValues_orientation)
    init_speed3d(predict) if first_time == True else calculateNewCoordonates(predict, newValues_acceleration, newValues_orientation)

        
        
        
def mainLoop(udp_socket):
    predict: Motion = Motion(position=[Axis(X=0, Y=0, Z=0)], speed=[0.0], speed3d=[Axis(X=0, Y=0, Z=0)], acceleration=[Axis(X=0, Y=0, Z=0)], direction=[EulerAngles(ψ=0, θ=0, φ=0)])
    debug: Motion = Motion(position=[Axis(X=0, Y=0, Z=0)], speed=[0.0], speed3d=[Axis(X=0, Y=0, Z=0)], acceleration=[Axis(X=0, Y=0, Z=0)], direction=[EulerAngles(ψ=0, θ=0, φ=0)])
    first_time = True
    data = b""
    while True:
        while data != "MSG_START".encode():
            try:
                data, _ = udp_socket.recvfrom(1024)
            except TimeoutError as e:
                print("Error:", e.args)
                exit(1)
        getData(udp_socket, predict, debug, first_time)
        print(f"predicted position: X={predict.position[-1].X} Y={predict.position[-1].Y} Z={predict.position[-1].Z}")
        udp_socket.sendto(f"{predict.position[-1].X} {predict.position[-1].Y} {predict.position[-1].Z}".encode(), ("127.0.0.1", 4242))
        first_time = False

def ft_kalman():
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    adresse, port = '0.0.0.0', 0
    buff = []
    udp_socket.bind((adresse, port))
    udp_socket.settimeout(1.0)

    udp_socket.sendto("READY".encode(), ("127.0.0.1", 4242))
    mainLoop(udp_socket)
    udp_socket.close()

if __name__ == "__main__":
    ft_kalman()