#version 330 core

out vec4 Color;

in vec2 vRelPos;
in vec2 vTexCoord;

uniform sampler2D uTexture;
uniform bool uHasTex;
uniform vec4 uColor;
uniform float uRoundEdge;

void edge_rounding(){
    // rounded edges
    vec2 v = vRelPos - vec2(0.8,0.8);   // top right
    if ((v.x > 0 && v.y > 0) && length(v) > 0.2){
        discard;
    }
    v = vRelPos - vec2(-0.8,-0.8);      // bottom left
    if ((v.x < 0 && v.y < 0) && length(v) > 0.2){
        discard;
    }
    v = vRelPos - vec2(-0.8,0.8);      // top left
    if ((v.x < 0 && v.y > 0) && length(v) > 0.2){
        discard;
    }
    v = vRelPos - vec2(0.8,-0.8);      // bottom right
    if ((v.x > 0 && v.y < 0) && length(v) > 0.2){
        discard;
    }
}

void main()
{

    if (uRoundEdge > 0){
        edge_rounding();
    }

    if (!uHasTex){
        Color = uColor;
    } else{
        Color = texture2D(uTexture, vTexCoord);
    }
}