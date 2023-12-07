use bevy::{prelude::*, render::view::RenderLayers};
pub use bevy_blueprints::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[bevy_blueprints(
	bundle = pub PlayerBundle,
	blueprint = pub PlayerBlueprint
)]
pub struct Player {
    #[blueprint(derived)]
    children: Vec<Transform>,

    #[blueprint(required)]
    #[bundle(sync)]
    transform: Transform,

    #[bundle(derived)]
    optimizable_shape: OptimizedShape,

    #[bundle(derived)]
    pbr_bundle: PbrBundle,

    #[bundle(value = Name::new("PlayerBundle"))]
    name: Name,

    #[bundle(value = RenderLayers::default())]
    render_layers: RenderLayers,
}

#[derive(Debug)]
enum OptimizedShape {
    Sphere { radius: f32 },
    Cube { length: f32 },
}

// produces

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
	transform: Transform,
	optimizable_shape: OptimizedShape,
	optimizable_shape_ignore: ::bevy_replicon::replicon_core::replication_rules::Ignored<OptimizedShape>,
}