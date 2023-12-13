from dataclasses import dataclass

@dataclass
class Axis:
    X: float
    Y: float
    Z: float

@dataclass
class EulerAngles:
    ψ: float
    θ: float
    φ: float

@dataclass
class Motion:
    position: list[Axis]
    speed: list[float]
    speed3d: list[Axis]
    acceleration: list[Axis]
    direction: list[EulerAngles]
    
    def __repr__(self):
        position = self.position[-1].__repr__()
        speed3d = self.speed3d[-1].__repr__()
        acceleration = self.acceleration[-1].__repr__()
        direction = self.direction[-1].__repr__()
        return "position:" + position + " \nspeed:" + str(self.speed[-1]) + "\nspeed3d:" + speed3d + "\nacceleration:" + acceleration + "\ndirection:" + direction