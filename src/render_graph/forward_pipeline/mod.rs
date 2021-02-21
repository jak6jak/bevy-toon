use bevy::asset::{Assets, HandleUntyped};
use bevy::reflect::TypeUuid;
use bevy::render::{
    pipeline::{BlendFactor, BlendOperation, ColorWrite, CompareFunction, PipelineDescriptor},
    shader::{Shader, ShaderStage, ShaderStages},
    texture::TextureFormat,
};

pub const FORWARD_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771389);
