fn create_map(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 10., 50.).looking_at(Vec3::ZERO, Vec3::Y)
    ));

    let ball_mesh = mesh_assets.add(Sphere::new(1.5));
    for h in 0..40 {
        let color = Color::hsl((h as f32 / 16.) * 360., 1., 0.5);
        let ball_material = material_assets.add(StandardMaterial {
            base_color: color,
            ..Default::default()
        });
        
        let random_height: f32 = rand::random_range(0.0..=30.0);
        commands.spawn((
            Transform::from_translation(Vec3::new((-20. + h as f32) * 4., random_height, -100.)),
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material),
        ));
    }

    let floor_mesh = mesh_assets.add(Cuboid::new(40.,5.,40.));
    let floor_material = material_assets.add(StandardMaterial {
        base_color: Color::linear_rgb(1., 0., 0.),
        ..Default::default()
    });
    commands.spawn((
        Transform::from_xyz(0.0, -5.1, 0.0),
        Mesh3d(floor_mesh),
        Collider::cuboid(11., 5., 11.),
        MeshMaterial3d(floor_material),
    ));
}
