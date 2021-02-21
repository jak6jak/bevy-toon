#version 450

in vec3 vPos;
in vec3 vNormal;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform MyMaterial_color {
    vec4 color;
};


struct Light {
    vec4 position;
    vec4 color;
}


void main() {
    o_Target = color;
}
