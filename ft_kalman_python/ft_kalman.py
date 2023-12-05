#!/usr/bin/python3

import socket
from dataclasses import dataclass
import matplotlib.pyplot as plt
import numpy

@dataclass
class Axis:
    X: float
    Y: float
    Z: float

@dataclass
class MotionTrue:
    position: Axis
    speed: float
    acceleration: Axis
    direction: Axis

@dataclass
class MotionPredict:
    position: Axis
    speed: float
    acceleration: Axis
    direction: Axis

def setValuesFirstTime(info, predict, true):
    print(info[1:])
    if info[0][14:] == "TRUE POSITION":
        (predict.position.X, predict.position.Y, predict.position.Z) = info[1:-1]
        (true.position.X, true.position.Y, true.position.Z) = info[1:-1]
    elif info[0][14:] == "SPEED":
        predict.speed = info[1]
        true.speed = info[1]
    elif info[0][14:] == "ACCELERATION":
        (predict.acceleration.X, predict.acceleration.Y, predict.acceleration.Z) = info[1:-1]
        (true.acceleration.X, true.acceleration.Y, true.acceleration.Z) = info[1:-1]
    elif info[0][14:] == "DIRECTION":
        (predict.direction.X, predict.direction.Y, predict.direction.Z) = info[1:-1]
        (true.direction.X, true.direction.Y, true.direction.Z) = info[1:-1]

def setValuesOtherTimes(info, predict, true):
    if info[0][14:] == "TRUE POSITION":
        (true.position.X, true.position.Y, true.position.Z) = info[1:-1]
    elif info[0][14:] == "SPEED":
        true.speed = info[1]
    elif info[0][14:] == "ACCELERATION":
        (predict.acceleration.X, predict.acceleration.Y, predict.acceleration.Z) = info[1:-1]
        (true.acceleration.X, true.acceleration.Y, true.acceleration.Z) = info[1:-1]
    elif info[0][14:] == "DIRECTION":
        (predict.direction.X, predict.direction.Y, predict.direction.Z) = info[1:-1]
        (true.direction.X, true.direction.Y, true.direction.Z) = info[1:-1]

def getData(udp_socket, predict, true, first_time):
    data = b""
    while data != "MSG_END".encode():
        data, _ = udp_socket.recvfrom(1024)
        mess = data.decode()
        info = mess.split("\n")
        setValuesFirstTime(info, predict, true) if first_time else setValuesOtherTimes(info, predict, true)
    
        
        
def mainLoop(udp_socket, graph):
    predict: MotionPredict = MotionPredict(position=Axis(X=0, Y=0, Z=0), speed=0, acceleration=Axis(X=0, Y=0, Z=0), direction=Axis(X=0, Y=0, Z=0))
    true: MotionTrue = MotionTrue(position=Axis(X=0, Y=0, Z=0), speed=0, acceleration=Axis(X=0, Y=0, Z=0), direction=Axis(X=0, Y=0, Z=0))
    first_time = True
    data = b""
    while True:
        while data != "MSG_START".encode():
            data, _ = udp_socket.recvfrom(1024)
            print(data)
        getData(udp_socket, predict, true, first_time)
        udp_socket.sendto(f"{predict.position.X} {predict.position.Y} {predict.position.Z}".encode(), ("127.0.0.1", 4242))
        first_time = False

def ft_kalman():
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    adresse, port = '0.0.0.0', 0
    buff = []
    udp_socket.bind((adresse, port))

    udp_socket.sendto("READY".encode(), ("127.0.0.1", 4242))
    mainLoop(udp_socket, graph)
    udp_socket.close()

if __name__ == "__main__":
    ft_kalman()