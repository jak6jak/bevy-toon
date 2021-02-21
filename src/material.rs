use bevy::asset::{self, Handle};
use bevy::reflect::TypeUuid;
use bevy::render::{color::Color, renderer::RenderResources, shader::ShaderDefs, texture::Texture};

#[derive(Debug, RenderResources, ShaderDefs, TypeUuid)]
#[uuid= "1f25edf8-db65-4655-b406-0dad100a5fee"]
pub struct StandardMaterial {
    pub albedo: Color,

}

impl Default for StandardMaterial {
    fn default() -> Self {
        StandardMaterial {
            albedo: Color::rgb(1.0,1.0,1.0)
        }
    }
}

impl From<Color> for StandardMaterial {
    fn from(color: Color) -> Self {
        StandardMaterial {
            albedo: color,
            ..Default::default()
        }
    }
}