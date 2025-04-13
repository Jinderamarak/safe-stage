#version 450

layout(location = 0) in vec3 FragPos; 
layout(location = 1) in vec3 Normal;

layout(location = 0) out vec4 outFragColor;

layout(push_constant) uniform constants{
	vec3 objectColor;
};

layout(binding = 0) uniform UniformBufferObject {
	mat4 projection;
	vec3 lightPos;
} ubo;

void main()
{
	float ambientStrength = 0.3;
	vec3 lightColor = vec3(1.0, 1.0, 1.0);
	vec3 ambient = ambientStrength * lightColor;

	vec3 norm = normalize(Normal);
	vec3 lightDir = normalize(ubo.lightPos - FragPos);

	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * lightColor;

	vec3 result = (ambient + diffuse) * objectColor;
	outFragColor = vec4(result, 1.0);
}