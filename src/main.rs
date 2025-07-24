use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::*;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default(), FpsControllerPlugin))
    .add_systems(Startup,(create_camera,create_map))
    .insert_resource(Time::<Fixed>::from_hz(60.))
    .add_systems(FixedUpdate, (bullet_despawn))
    .add_systems(Update, (spawn_bullet, shoot_bullet.before(spawn_bullet), apply_velocity))
    .add_event::<BulletSpawn>()
    .run();
}

include!("camera.rs");
include!("map.rs");
include!("shoot.rs");