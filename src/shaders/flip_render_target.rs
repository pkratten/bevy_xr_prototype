use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget, render_graph, render_resource::PipelineCache, renderer::RenderContext,
    },
    utils::{HashMap, HashSet},
};

struct FlipRenderTargetPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct FlipRenderTargetLabel;

impl Plugin for FlipRenderTargetPlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins(ExtractResourcePlugin::<FlipRenderTargets>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(FlipRenderTargetLabel, FlipRenderTargetNode::default());
        render_graph.add_node_edge(
            bevy::render::graph::CameraDriverLabel,
            FlipRenderTargetLabel,
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<GameOfLifePipeline>();
    }
}

enum FlipDirection {
    X,
    Y,
    XY,
}

#[derive(Resource, Clone, Deref, ExtractResource)]
struct FlipRenderTargets {
    render_targets: HashMap<RenderTarget, FlipDirection>,
}

struct FlipRenderTargetNode;

impl render_graph::Node for FlipRenderTargetNode {
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<FlipRenderTargetPipeline>();

        Ok(())
    }
}
