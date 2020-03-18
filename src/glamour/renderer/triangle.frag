#version 410 core

out vec4 Color;

uniform vec4 u_Color;

void main()
{
    Color = u_Color;
}
