#version 450

layout (location = 0) in vec3 VertPos;
layout (location = 1) in vec3 VertNormal;

layout (location = 0) out vec3 FragPos;
layout (location = 1) out vec3 FragNormal;

layout (binding = 0) uniform UniformBufferObject {
    mat4 projection;
    vec3 lightPosition;
    vec3 lightColor;
    float lightStrength;
};

void main()
{
    gl_Position = projection * vec4(VertPos, 1.0);
    FragPos = VertPos;
    FragNormal = VertNormal;
}