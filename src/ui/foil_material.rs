// Foil shader material for max-tier upgraded cards
// Creates an animated holographic/iridescent effect

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;

/// Custom UI material for holographic foil effect
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct FoilMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub intensity: f32,
    // Padding for 16-byte alignment
    #[uniform(0)]
    pub _padding: Vec2,
}

impl Default for FoilMaterial {
    fn default() -> Self {
        Self {
            time: 0.0,
            intensity: 1.0,
            _padding: Vec2::ZERO,
        }
    }
}

impl UiMaterial for FoilMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/foil.wgsl".into()
    }
}

/// Resource to track foil animation time
#[derive(Resource, Default)]
pub struct FoilAnimationTime(pub f32);

/// Marker component for cards that need foil overlay
#[derive(Component)]
pub struct FoilCard;

/// Marker to prevent re-adding foil overlay
#[derive(Component)]
pub struct HasFoilOverlay;

/// System to update foil material time for animation
pub fn update_foil_materials(
    time: Res<Time>,
    mut foil_time: ResMut<FoilAnimationTime>,
    mut materials: ResMut<Assets<FoilMaterial>>,
) {
    foil_time.0 += time.delta_secs();

    // Update all foil materials with current time
    for (_, material) in materials.iter_mut() {
        material.time = foil_time.0;
    }
}

/// System to add foil overlays to cards marked with FoilCard
pub fn spawn_foil_overlays(
    mut commands: Commands,
    mut materials: ResMut<Assets<FoilMaterial>>,
    foil_time: Res<FoilAnimationTime>,
    query: Query<Entity, (With<FoilCard>, Without<HasFoilOverlay>)>,
) {
    for entity in query.iter() {
        // Create the foil material
        let foil_handle = materials.add(FoilMaterial {
            time: foil_time.0,
            intensity: 1.0,
            _padding: Vec2::ZERO,
        });

        // Use try_insert to safely handle despawned entities
        commands.entity(entity).try_insert(HasFoilOverlay);

        // Spawn foil overlay as child - use queue with world access for safe entity check
        commands.queue(move |world: &mut World| {
            // Check if entity still exists
            if world.get_entity(entity).is_err() {
                return;
            }

            // Spawn the child
            let child = world.spawn((
                MaterialNode(foil_handle),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            )).id();

            // Add as child if parent still exists
            if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
                entity_mut.add_child(child);
            } else {
                // Parent gone, despawn orphaned child
                let _ = world.despawn(child);
            }
        });
    }
}

/// Plugin to set up foil material system
pub struct FoilMaterialPlugin;

impl Plugin for FoilMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<FoilMaterial>::default())
            .init_resource::<FoilAnimationTime>()
            .add_systems(Update, (update_foil_materials, spawn_foil_overlays));
    }
}
