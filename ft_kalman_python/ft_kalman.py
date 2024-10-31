#!/home/pgoudet/work/ft_kalman/ft_kalman_python/venv/bin/python3
import socket
import matplotlib.pyplot as plt
from structs import Axis, Motion, EulerAngles
from kalman_compute import calculateNewCoordonates, computeVelocity
from init_values import setValuesFirstTime, setValuesOtherTimes, endProg
from filterpy.kalman import KalmanFilter
from filterpy.common import Q_discrete_white_noise
import numpy as np
import re
from datetime import datetime
import argparse

parser = argparse.ArgumentParser(description="filtre de kalman avec option de debug.")
TIME: datetime = datetime(1970, 1, 1)
GREEN = "\033[92m"
RESET = "\033[0m"
σ_ACC = 10**-3
σ_GYR = 10**-2
σ_GPS = 10**-1
DT = 1/100

def getData(udp_socket, predict, debug, first_time, f, variances, args):
    global TIME, DT
    f: KalmanFilter
    newValues_acceleration = Axis(X=0, Y=0, Z=0)
    newValues_orientation = EulerAngles(φ=0, θ=0, ψ=0)
    newValues_position = Axis(X=0, Y=0, Z=0)
    data = b""
    while data != "MSG_END".encode():
        try:
            data, _ = udp_socket.recvfrom(1024)
        except TimeoutError as e:
            endProg(udp_socket, predict, debug, variances, e, TIME)
        mess = data.decode()
        info = mess.split("\n")
        if re.match("\[\d{2}:\d{2}:\d{2}\.\d{3}\]", info[0][:14]):
            TIME = datetime.strptime(info[0][:14], "[%H:%M:%S.%f]").time()
        setValuesFirstTime(info, predict, debug) if first_time else setValuesOtherTimes(info, debug, newValues_acceleration, newValues_orientation, newValues_position)
    if first_time == False:
        calculateNewCoordonates(predict, newValues_acceleration, newValues_orientation, f, args, newValues_position, debug, TIME)
        P_matrix = f.P
        v = P_matrix.diagonal()
        variances = np.vstack([variances, np.array([v[0], v[1], v[2], v[3], v[4]])])
    else:
        vel, _ = computeVelocity(predict.direction[-1], predict.acceleration[-1], np.array([[predict.speed[-1]], [0], [0]]), DT)
        vel = vel.reshape(3, )
        predict.speed3d.append(Axis(vel[0], vel[1], vel[2]))
        f.x = np.array([[predict.position[-1].X], [predict.position[-1].Y], [predict.position[-1].Z],
                        [predict.speed3d[-1].X], [predict.speed3d[-1].Y], [predict.speed3d[-1].Z]])
    return variances

def setMatrix():
    global DT

    f = KalmanFilter(dim_x=6, dim_z=3)
    f.x = None
    #state matrix
    f.F = np.array([
        [1, 0, 0, DT, 0 , 0 ],
        [0, 1, 0, 0 , DT, 0 ],
        [0, 0, 1, 0 , 0 , DT],
        [0, 0, 0, 1 , 0 , 0 ],
        [0, 0, 0, 0 , 1 , 0 ],
        [0, 0, 0, 0 , 0 , 1 ],
    ])
    
    #measurement matrix
    f.H = np.array([  
                    [1, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0]
                ])
    
    # control matrix
    f.B =  np.array([ 
                    [(DT**2) / 2, 0          , 0          ],
                    [0          , (DT**2) / 2, 0          ],
                    [0          , 0          , (DT**2) / 2],
                    [DT         , 0          , 0          ],
                    [0          , DT         , 0          ],
                    [0          , 0          , DT         ]
                ])

    # process noise matrix
    continous_position = np.array([
        [DT**2 / 2, 0        , 0        , 0, 0, 0],
        [0        , DT**2 / 2, 0        , 0, 0, 0],
        [0        , 0        , DT**2 / 2, 0, 0, 0],
        [0        , 0        , 0        , 0, 0, 0],
        [0        , 0        , 0        , 0, 0, 0],
        [0        , 0        , 0        , 0, 0, 0]
    ])

    continous_acceleration = np.array([
        [DT**2 / 2, 0        , 0        , DT**2 / 2, 0        , 0        ],
        [0        , DT**2 / 2, 0        , 0        , DT**2 / 2, 0        ],
        [0        , 0        , DT**2 / 2, 0        , 0        , DT**2 / 2],
        [0        , 0        , 0        , DT       , 0        , 0        ],
        [0        , 0        , 0        , 0        , DT       , 0        ],
        [0        , 0        , 0        , 0        , 0        , DT       ]
    ])

    continous_speed = np.array([
        [0, 0, 0, DT**2 / 2, 0        , 0        ],
        [0, 0, 0, 0        , DT**2 / 2, 0        ],
        [0, 0, 0, 0        , 0        , DT**2 / 2],
        [0, 0, 0, DT       , 0        , 0        ],
        [0, 0, 0, 0        , DT       , 0        ],
        [0, 0, 0, 0        , 0        , DT       ]
    ])

    var_p = σ_GPS ** 2 
    var_s = σ_GYR ** 2 + σ_ACC ** 2 * DT
    var_a = σ_ACC ** 2

    noise_position = np.array([
        [var_p, 0    , 0    , 0, 0, 0],
        [0    , var_p, 0    , 0, 0, 0],
        [0    , 0    , var_p, 0, 0, 0],
        [0    , 0    , 0    , 0, 0, 0],
        [0    , 0    , 0    , 0, 0, 0],
        [0    , 0    , 0    , 0, 0, 0],
    ])

    noise_acceleration = np.array([
        [DT**2 / 2 * var_a, 0                , 0                , DT**2 / 2 * var_a, 0                , 0                ],
        [0                , DT**2 / 2 * var_a, 0                , 0                , DT**2 / 2 * var_a, 0                ],
        [0                , 0                , DT**2 / 2 * var_a, 0                , 0                , DT**2 / 2 * var_a],
        [0                , 0                , 0                , DT * var_a       , 0                , 0                ],
        [0                , 0                , 0                , 0                , DT * var_a       , 0                ],
        [0                , 0                , 0                , 0                , 0                , DT * var_a       ],
    ])
    
    noise_speed = np.array([
        [0, 0, 0, DT * var_s, 0         , 0         ],
        [0, 0, 0, 0         , DT * var_s, 0         ],
        [0, 0, 0, 0         , 0         , DT * var_s],
        [0, 0, 0,var_s      , 0         , 0         ],
        [0, 0, 0, 0         ,var_s      , 0         ],
        [0, 0, 0, 0         , 0         ,var_s      ],
    ])

    noise = np.dot(continous_position, noise_position) + np.dot(continous_acceleration, noise_acceleration) + np.dot(continous_speed, noise_speed)
    f.Q = np.dot(np.dot(f.F, noise), f.F.T)
    # integrer le tout
    f.Q *= DT

    # measurement uncertainty
    f.R = np.array([  
                    [σ_GPS**2, 0       , 0       ],
                    [0       , σ_GPS**2, 0       ],
                    [0       , 0       , σ_GPS**2]
                ])
    
    f.P = np.array([
                    [σ_GPS**2, 0       , 0       , 0                       , 0                       , 0                       ],
                    [0       , σ_GPS**2, 0       , 0                       , 0                       , 0                       ],
                    [0       , 0       , σ_GPS**2, 0                       , 0                       , 0                       ],
                    [0       , 0       , 0       , σ_ACC**2 + σ_GYR**2 * DT, 0                       , 0                       ],
                    [0       , 0       , 0       , 0                       , σ_ACC**2 + σ_GYR**2 * DT, 0                       ],
                    [0       , 0       , 0       , 0                       , 0                       , σ_ACC**2 + σ_GYR**2 * DT]
                ])


    return f

        
def mainLoop(udp_socket):
    global TIME
    
    args = parser.parse_args()
    predict: Motion = Motion(position=[], speed=[], speed3d=[], acceleration=[], direction=[])
    debug: Motion = Motion(position=[], speed=[], speed3d=[], acceleration=[], direction=[])
    variances = np.empty((0, 5), np.float32)
    first_time = True
    data = b""
    f = setMatrix()
    while True:
        while data != "MSG_START".encode():
            try:
                data, _ = udp_socket.recvfrom(1024)
            except TimeoutError as e:
                endProg(udp_socket, predict, debug, variances, e, TIME)
        variances = getData(udp_socket, predict, debug, first_time, f, variances, args)
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
    parser.add_argument('--debug', '-d', action='store_true', help='Activer le mode debug')
    ft_kalman()