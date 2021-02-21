use crate::{material::StandardMaterial, render_graph::FORWARD_PIPELINE_HANDLE};
use bevy::asset::Handle;
use bevy::ecs::Bundle;
use bevy::render::{
    draw::Draw,
    mesh::Mesh,
    pipeline::{RenderPipeline, RenderPipelines},
    prelude::Visible,
    render_graph::base::MainPass,
};
use bevy::transform::prelude::{GlobalTransform, Transform};

/// A component bundle for "pbr mesh" entities
#[derive(Bundle)]
pub struct ToonBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for ToonBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                FORWARD_PIPELINE_HANDLE.typed(),
            )]),
            mesh: Default::default(),
            visible: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}