#version 330 core

layout (location = 0) in vec3 aPosition;

uniform mat4 aModel;
uniform mat4 aView;
uniform mat4 aProjection;

out vec3 FragPosition;

void main()
{
    FragPosition = vec3(aModel * vec4(aPosition, 1.0));
    gl_Position = aProjection * aView * vec4(FragPosition, 1.0);
}
