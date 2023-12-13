#!/usr/bin/python3

import numpy as np

def f(mu, sigma, x):
    coefficient = 1.0 / np.sqrt(2.0 * np.pi* sigma)
    expo = np.exp(-0.5 * (x - mu) ** 2 / sigma)
    return coefficient * expo