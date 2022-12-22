# Bevy Ray Marching Project
This is a distance-aided ray marcher created using the Bevy game engine.

Currently, the ray marcher creates a custom screen space quad with a customer shader. All the ray marching logic exists in the fragment shader. There is a single sphere that is ray marched using a displacement along with time to create a distortion effect. The ray marcher also automatically adjusts when the screen is resized to take into account the aspect ratio.

The current output of the project is shown below:
![](https://github.com/lukeduball/assets/output/displacement_sphere.gif)