fn create_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    let logical = commands
        .spawn((
            Collider::capsule_y(1.0, 0.5),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Friction::coefficient(0.0),
            Restitution::coefficient(0.0),
            GravityScale(0.0),
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            Ccd { enabled: true },
            Transform::from_xyz(0.0, 3.0, 0.0),
            GlobalTransform::default(),
            LogicalPlayer,
            FpsControllerInput::default(),
            FpsController::default(),
            CameraConfig {
                height_offset: -0.5,
            },
        ))
        .id();

    // Spawn the camera as a child
    commands.entity(logical).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            RenderPlayer {
                logical_entity: logical,
            },
        ));
    });

    let mut window = windows.single_mut().unwrap();
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}