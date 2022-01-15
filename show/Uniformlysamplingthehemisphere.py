from mpl_toolkits.mplot3d import Axes3D
import matplotlib.pyplot as plt
import numpy as np

fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

# For each set of style and range settings, plot n random points in the box
# defined by x in [23, 32], y in [0, 100], z in [zlow, zhigh].
for i in range(0,200):
    z = np.random.rand()
    s = np.random.rand()
    r = np.sqrt(1.0 - z * z )
    phi = 2.0 * np.pi * s
    x = np.cos(phi) * r
    y = np.sin(phi) * r
    ax.scatter(x, y, r, c='b', marker='o')

ax.set_xlabel('X Label')
ax.set_ylabel('Y Label')
ax.set_zlabel('Z Label')

plt.show()


for i in range(0,200):
    r1 = np.random.rand()
    r2 = np.random.rand()
    r = np.sqrt(1.0 - z * z )
    phi = 2.0 * np.pi * r1
    x = np.cos(phi) * r
    y = np.sin(phi) * r
    ax.scatter(x, y, z, c='b', marker='o')

ax.set_xlabel('X Label')
ax.set_ylabel('Y Label')
ax.set_zlabel('Z Label')

plt.show()