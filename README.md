# Voxel Sand Simulation

[Web Demo](https://sand.saahil-gupta.com/)

![image](/img/sand.png)

A voxel sand simulation based off the original 2D pixel sand simulation, available on [my website](https://www.saahil-gupta.com/sand/). 
Note that 3D cellular automata is very inefficient and does not scale well, especially on WebGL. The dimensions of the box have been set to 50x50x30, but the application will likely slow down when reaching a large number of voxels. For best performance, please use a browser like Chrome or Edge.
Actively exploring optimizations with CUDA.
