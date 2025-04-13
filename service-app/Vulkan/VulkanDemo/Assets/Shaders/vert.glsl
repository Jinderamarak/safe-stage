#version 450

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;

layout(location = 0) out vec3 FragPos;
layout(location = 1) out vec3 Normal;

layout(push_constant) uniform constants{
	vec3 objectColor;
};

layout(binding = 0) uniform UniformBufferObject {
	mat4 projection;
	vec3 lightPos;
} ubo;

void main()
{
	gl_Position = ubo.projection * vec4(aPos, 1.0);
	FragPos = aPos;
	Normal = aNormal;
}