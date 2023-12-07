import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np

# Créez les données
x = np.linspace(0, 10, 100)
y = np.sin(x)
z = np.cos(x)

# Calculez l'incertitude
incertitude_x = 0.1 * x
incertitude_y = 0.1 * y
incertitude_z = 0.1 * z

# Créez l'axe 3D
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

# Tracez la ligne originale
ax.plot(x, y, z)

# Tracez la zone d'incertitude
ax.plot_trisurf(x + incertitude_x, y + incertitude_y, z + incertitude_z, linewidth=0, antialiased=False)

# Affichez le graphique
plt.show()