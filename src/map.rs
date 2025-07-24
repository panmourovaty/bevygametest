fn create_map(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLight::default());
    let ball_mesh = mesh_assets.add(Sphere::new(1.));
    for h in 0..16 {
        let color = Color::hsl((h as f32 / 16.) * 360., 1., 0.5);
        let ball_material = material_assets.add(StandardMaterial {
            base_color: color,
            ..Default::default()
        });
        commands.spawn((
            Transform::from_translation(Vec3::new((-8. + h as f32) * 2., 5., -50.)),
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material),
        ));
    }

    let floor_mesh = mesh_assets.add(Cuboid::new(20.,1.,20.));
    let floor_material = material_assets.add(StandardMaterial {
        base_color: Color::linear_rgb(255., 1., 1.),
        ..Default::default()
    });
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(floor_mesh),
        Collider::cuboid(20., 1., 20.),
        MeshMaterial3d(floor_material),
    ));
}
