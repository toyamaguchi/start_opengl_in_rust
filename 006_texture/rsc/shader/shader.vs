#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 aModel;
uniform mat4 aView;
uniform mat4 aProjection;
uniform float aAlpha;

out float Alpha;
out vec3 FragPosition;
out vec3 Normal;
out vec2 TexCoords;

void main()
{
    Alpha = aAlpha;
    FragPosition = vec3(aModel * vec4(aPosition, 1.0));
    Normal = mat3(transpose(inverse(aModel))) * aNormal;
    TexCoords = aTexCoords;
    gl_Position = aProjection * aView * vec4(FragPosition, 1.0);
}
