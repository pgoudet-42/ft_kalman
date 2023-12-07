import numpy as np

def getTransitionMatrix(dt):
    transition_matrice = np.array([
        [1, 0, 0, dt, 0, 0],
        [0, 1, 0, 0, dt, 0],
        [0, 0, 1, 0, 0, dt],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1],
    ])
    return transition_matrice

def getStateVector(predict):
    state_vector = np.array([
        predict.position[-1].X,
        predict.position[-1].Y,
        predict.position[-1].Z,
        predict.acceleration[-1].X,
        predict.acceleration[-1].Y,
        predict.acceleration[-1].Z,
        predict.direction[-1].X,
        predict.direction[-1].Y,
        predict.direction[-1].Z,
    ])
    return state_vector

def getInputVariable(new_acceleration, new_orientation):
    inut_variables = np.array([
        0,
        0,
        0,
        new_acceleration.X,
        new_acceleration.Y,
        new_acceleration.Z,
        new_orientation.X,
        new_orientation.Y,
        new_orientation.Z,
    ])

def getControlMatrix():
    ...

def calculateNewCoordonates(predict, new_acceleration, new_orientation):
    predict.position[-1].X = predict.position[-1].X
    predict.position[-1].Y = predict.position[-1].Y
    predict.position[-1].Z = predict.position[-1].Z
