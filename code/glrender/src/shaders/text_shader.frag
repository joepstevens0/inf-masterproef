#version 330 core

out vec4 Color;

in vec2 vTexCoord;
uniform sampler2D uTexture;
uniform bool uHasTex;
uniform vec4 uColor;


void main()
{
    if (!uHasTex){
        Color = uColor;
    } else{
        float c = texture2D(uTexture, vTexCoord).r;
        if (c > 0){
            Color = c*uColor;
        } else
            discard;
    }
}