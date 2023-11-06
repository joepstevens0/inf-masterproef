#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in float Size;

uniform mat4 uView;
uniform mat4 uProj;

out vec4 vColor;

void main(){
    gl_PointSize = Size;
    gl_Position = uProj*uView*vec4(Position, 1);
    vColor = Color;
}