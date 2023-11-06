#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 TexCoord;

uniform uvec2 uScreenSize;
uniform ivec2 uOffset;
uniform uvec2 uSize;

out vec2 vTexCoord;
out vec2 vRelPos;

void main(){
    vec2 size = vec2(uSize)/vec2(uScreenSize);
    vec2 offset = 2.*vec2(uOffset)/vec2(uScreenSize);
    offset.y = 2. - offset.y  - 2*size.y;

    vec2 p = Position + vec2(1,1);
    p = (p*size )+offset;
    p -= vec2(1,1);
    gl_Position = vec4(p, 0, 1.0);
    vTexCoord = TexCoord;
    vRelPos = Position;
}