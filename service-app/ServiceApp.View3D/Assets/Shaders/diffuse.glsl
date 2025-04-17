#version 450

layout (location = 0) in vec3 FragPos;
layout (location = 1) in vec3 FragNormal;

layout (location = 0) out vec4 FragColor;

layout (push_constant) uniform constants {
    vec3 objectColor;
};

layout (binding = 0) uniform UniformBufferObject {
    mat4 projection;
    vec3 lightPosition;
    vec3 lightColor;
    float lightStrength;
};

void main()
{
    vec3 ambient = lightStrength * lightColor;

    vec3 norm = normalize(FragNormal);
    vec3 lightDir = normalize(lightPosition - FragPos);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 result = (ambient + diffuse) * objectColor;
    FragColor = vec4(result, 1.0);
}