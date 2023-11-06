#version 330 core


in vec2 vTexCoord;
in vec3 vNormal;
in vec3 vFragPos;

uniform uint uID;

out uint oID;

void main()
{
	oID = uID;
}