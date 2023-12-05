import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from matplotlib.animation import FuncAnimation

# Fonction pour générer des données 3D
def generate_data(t):
    x = np.sin(t)
    y = np.cos(t)
    z = t
    return x, y, z

# Initialisation du graphe 3D
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

# Paramètres de l'animation
num_frames = 100
t_values = np.linspace(0, 20, num_frames)

# Fonction de mise à jour de l'animation
def update(frame):
    ax.cla()  # Efface le graphe à chaque itération
    ax.set_title(f'Frame {frame}/{num_frames}')
    
    # Génère les nouvelles données pour le nouveau frame
    x, y, z = generate_data(t_values[frame])

    # Crée le graphe 3D
    ax.scatter(x, y, z, c='r', marker='o')
    ax.set_xlabel('X Label')
    ax.set_ylabel('Y Label')
    ax.set_zlabel('Z Label')

# Crée l'animation
animation = FuncAnimation(fig, update, frames=num_frames, interval=100)

# Affiche l'animation
plt.show()