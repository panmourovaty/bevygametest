#[derive(Event)]
struct BulletSpawn {
    position: Vec3,
    velocity: Vec3
}

fn spawn_bullet(
    mut events: EventReader<BulletSpawn>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let bullet_mesh = mesh_assets.add(Cuboid::new(0.01, 0.01, 0.04));

    for spawn in events.read() {
        let color = Color::linear_rgb(255., 255., 0.);
        let bullet_material = material_assets.add(StandardMaterial {
            base_color: color,
            ..Default::default()
        });

        // 🔄 Compute rotation to face velocity direction
        let direction = spawn.velocity.normalize_or_zero();
        let rotation = if direction.length_squared() > 0.0 {
            Quat::from_rotation_arc(Vec3::Z, direction)
        } else {
            Quat::IDENTITY
        };

        commands.spawn((
            Transform {
                translation: spawn.position,
                rotation,
                ..Default::default()
            },
            Mesh3d(bullet_mesh.clone()),
            MeshMaterial3d(bullet_material),
            ShootVelocity(spawn.velocity),
            BulletLifetime(Timer::from_seconds(4.0, TimerMode::Once)),
        ));
    }
}

fn shoot_bullet(
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<&GlobalTransform, (With<Camera3d>, With<RenderPlayer>)>,
    mut spawner: EventWriter<BulletSpawn>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let transform = match camera_query.single() {
        Ok(t) => t,
        Err(e) => {
            warn!("Camera transform not found: {:?}", e);
            return;
        }
    };

    let forward = transform.forward();
    let spawn_position = transform.translation() + forward * 1.;

    spawner.write(BulletSpawn {
        position: spawn_position,
        velocity: forward * 20.0,
    });
}


#[derive(Component)]
struct ShootVelocity(Vec3);

fn apply_velocity(mut objects: Query<(&mut Transform, &ShootVelocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut objects {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

#[derive(Component)]
struct BulletLifetime(Timer);

fn bullet_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BulletLifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.0.tick(time.delta());

        if lifetime.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}