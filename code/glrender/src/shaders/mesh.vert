#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProj;

uniform uvec2 uScreenSize;
uniform ivec2 uOffset;
uniform uvec2 uSize;

out vec2 vTexCoord;
out vec3 vNormal;
out vec3 vFragPos;

void main(){

    gl_Position = uProj*uView*uModel*vec4(Position, 1);
    vTexCoord = TexCoord;
    vNormal = mat3(transpose(inverse(uModel)))*Normal;
    vFragPos = vec3(uModel*vec4(Position, 1));
}