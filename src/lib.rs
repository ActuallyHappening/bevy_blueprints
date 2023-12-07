use bevy::{prelude::*, ecs::system::SystemParam};

pub use bevy_blueprints_macros::*;

pub trait Blueprint: Component + std::fmt::Debug {
	type For: Bundle + std::fmt::Debug;
	type ReducedBundle: std::fmt::Debug + Into<Self::For>;
	type SystemParam: SystemParam + Send + Sync + 'static + std::fmt::Debug;

	fn stamp_into_bundle(&self, system_param: &mut Self::SystemParam) -> Self::For;
}

