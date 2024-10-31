from dataclasses import dataclass
import numpy as np

@dataclass
class Axis:
    X: float
    Y: float
    Z: float

    def __mul__(self, scalar):
        if isinstance(scalar, (int, float)):
            return Axis(self.X * scalar, self.Y * scalar, self.Z * scalar)
        else:
            return NotImplemented
    
    def __rmul__(self, scalar):
        return self.__mul__(scalar)

    def __add__(self, other):
        if isinstance(other, Axis):
            return Axis(self.X + other.X, self.Y + other.Y, self.Z + other.Z)
        else:
            return NotImplemented

    def __radd__(self, other):
        return self.__add__(other)
    
    def __sub__(self, other):
        if isinstance(other, Axis):
            return Axis(self.X - other.X, self.Y - other.Y, self.Z - other.Z)
        else:
            return NotImplemented

    def __rsub__(self, other):
        if isinstance(other, Axis):
            return Axis(other.x - self.X, other.y - self.Y, other.z - self.Z)
        else:
            return NotImplemented

    def __mul__(self, scalar):
        if isinstance(scalar, (int, float)):
            return Axis(self.X * scalar, self.Y * scalar, self.Z * scalar)
        else:
            return NotImplemented
        
    def __rmul__(self, scalar):
        return self.__mul__(scalar)
    
    def __truediv__(self, scalar):
        if isinstance(scalar, (int, float)):
            if scalar != 0:
                return Axis(self.X / scalar, self.Y / scalar, self.Z / scalar)
            else:
                raise ValueError("Cannot divide by zero")
        else:
            return NotImplemented

    def __rtruediv__(self, scalar):
        return self.__truediv__(scalar)

    def __repr__(self):
        return f"Axis({self.X}, {self.Y}, {self.Z})"
    
    def to_nparray(self):
        return np.array([
            [self.X],
            [self.Y],
            [self.Z]
        ])

    def is_notinitialized(self):
        return True if self.X == 0 and self.Y == 0 and self.Z == 0 else False
            
        

@dataclass
class EulerAngles:
    φ: float
    θ: float
    ψ: float

@dataclass
class Motion:
    position: list[Axis]
    speed: list[float]
    speed3d: list[Axis]
    acceleration: list[Axis]
    direction: list[EulerAngles]
    
    def __repr__(self):
        position = self.position[-1].__repr__()
        acceleration = self.acceleration[-1].__repr__()
        direction = self.direction[-1].__repr__()
        speed3d = self.speed3d[-1].__repr__()
        return "position:" + position + " \nspeed:" + str(self.speed[-1]) +  "\nspeed3d:" + speed3d + "\nacceleration:" + acceleration + "\ndirection:" + direction

    def to_nparray(self):
        return np.array([
            [self.position[-1].X],
            [self.position[-1].Y],
            [self.position[-1].Z],
            [self.speed[-1]],
            [self.speed[-1]],
            [self.speed[-1]],
            [self.acceleration[-1].X],
            [self.acceleration[-1].Y],
            [self.acceleration[-1].Z],
        ])