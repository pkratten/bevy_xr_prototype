use std::collections::HashSet;

use bevy::{
    core_pipeline::core_3d,
    prelude::*,
    render::{
        camera::ExtractedCamera,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_graph::{RenderGraphApp, ViewNode, ViewNodeRunner},
        render_resource::{Extent3d, ImageCopyTexture, Origin3d, Texture, TextureAspect},
        view::ViewTarget,
        RenderApp,
    },
};

#[derive(Component, Default, ExtractComponent, Clone, Deref, DerefMut)]
pub struct CopyView(Vec<CopyDestination>);

#[derive(Clone)]
pub struct CopyDestination(pub Texture, pub UVec2);

pub struct CopyViewPlugin;

impl Plugin for CopyViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ExtractComponentPlugin::<CopyView>::default(),));

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_render_graph_node::<ViewNodeRunner<CopyTextureNode>>(
                    core_3d::graph::NAME,
                    CopyTextureNode::NAME,
                )
                .add_render_graph_edges(
                    core_3d::graph::NAME,
                    &[core_3d::graph::node::UPSCALING, CopyTextureNode::NAME],
                );
        }
    }
}

#[derive(Default)]
struct CopyTextureNode;
impl CopyTextureNode {
    pub const NAME: &'static str = "copy_view";
}

impl ViewNode for CopyTextureNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static CopyView,
        &'static ExtractedCamera,
    );

    fn run(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        (view, target, camera): bevy::ecs::query::QueryItem<Self::ViewQuery>,
        _world: &World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        for CopyDestination(destination, offset) in target.iter() {
            if let Some(size) = camera.physical_target_size {
                render_context.command_encoder().copy_texture_to_texture(
                    ImageCopyTexture {
                        texture: view.main_texture(),
                        mip_level: 0,
                        origin: Origin3d::ZERO,
                        aspect: TextureAspect::All,
                    },
                    ImageCopyTexture {
                        texture: destination,
                        mip_level: 0,
                        origin: Origin3d {
                            x: offset.x,
                            y: offset.y,
                            z: 0,
                        },
                        aspect: TextureAspect::All,
                    },
                    Extent3d {
                        width: size.x,
                        height: size.y,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }

        return Ok(());
    }
}
