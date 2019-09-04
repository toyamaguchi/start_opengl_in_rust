#version 330 core

struct Material {
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in float Alpha;
in vec3 FragPosition;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D screenTexture;
uniform vec3 viewPosition;
uniform Material material;
uniform Light light;

out vec4 FragColor;

void main()
{
    // ambient
    vec3 ambient = light.ambient * texture(screenTexture, TexCoords).rgb;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(-light.direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * texture(screenTexture, TexCoords).rgb;

    // specular
    vec3 viewDir = normalize(viewPosition - FragPosition);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * material.specular;

    vec3 result = ambient + diffuse + specular;

    FragColor = vec4(result, Alpha);
}
