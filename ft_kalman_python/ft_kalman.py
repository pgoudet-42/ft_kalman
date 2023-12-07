#!/usr/bin/python3

import socket
from dataclasses import dataclass
import matplotlib.pyplot as plt
import numpy
import time
from kalman_compute import calculateNewCoordonates

@dataclass
class Axis:
    X: float
    Y: float
    Z: float

@dataclass
class Motion:
    position: list[Axis]
    speed: float
    acceleration: list[Axis]
    direction: list[Axis]


NB_MSG_RCV = 0

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
        predict.direction.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
        debug.direction.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    return info[0][14:]

def setValuesOtherTimes(info, debug, acceleration, orientation):
    if info[0][14:] == "TRUE POSITION":
        debug.position.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "SPEED":
        debug.speed.append(float(info[1]))
    elif info[0][14:] == "ACCELERATION":
        acceleration.X, acceleration.Y, acceleration.Z = (float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2]))
        debug.acceleration.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    elif info[0][14:] == "DIRECTION":
        orientation.X, orientation.Y, orientation.Z = (float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2]))
        debug.direction.append(Axis(float(info[1:-1][0]), float(info[1:-1][1]), float(info[1:-1][2])))
    return info[0][14:]

def displayAndLeave(predict, debug):
    global NB_MSG_RCV

    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')
    print(NB_MSG_RCV)
    ax.plot(predict.position[-1].X, predict.position[-1].Y, predict.position[-1].Z)
    ax.plot(debug.position[-1].X, debug.position[-1].Y, debug.position[-1].Z)
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_zlabel('Z')
    ax.set_title('Graph of True position and Predicted position')
    # plt.show()
    exit(0)

def getData(udp_socket, predict, debug, first_time):
    newValues_acceleration: Axis = Axis(0,0,0)
    newValues_orientation: Axis = Axis(0,0,0)
    data = b""
    while data != "MSG_END".encode():
        try:
            data, _ = udp_socket.recvfrom(1024)
        except TimeoutError as e:
            print(e.__repr__())
            displayAndLeave(predict, debug)
        mess = data.decode()
        info = mess.split("\n")
        print(data)
        setValuesFirstTime(info, predict, debug) if first_time else setValuesOtherTimes(info, debug, newValues_acceleration, newValues_orientation)
    calculateNewCoordonates(predict, newValues_acceleration, newValues_orientation)
        
        
def mainLoop(udp_socket):
    global NB_MSG_RCV

    predict: Motion = Motion(position=[Axis(X=0, Y=0, Z=0)], speed=[0.0], acceleration=[Axis(X=0, Y=0, Z=0)], direction=[Axis(X=0, Y=0, Z=0)])
    debug: Motion = Motion(position=[Axis(X=0, Y=0, Z=0)], speed=[0.0], acceleration=[Axis(X=0, Y=0, Z=0)], direction=[Axis(X=0, Y=0, Z=0)])
    first_time = True
    data = b""
    while True:
        NB_MSG_RCV += 1
        while data != "MSG_START".encode():
            try:
                data, _ = udp_socket.recvfrom(1024)
            except TimeoutError as e:
                print("Error:", e.args)
                displayAndLeave(predict, debug)
        getData(udp_socket, predict, debug, first_time)
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