use bevy::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const CAMERA_SPEED: f32 = 4.0;

#[derive(Component)]
struct Player;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, (move_player, update_camera).chain());
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    let mut shapes: Vec<Handle<Mesh>> = Vec::new();

    for x in -100..100 {
        for y in -100..100 {
            let root_position = Vec2::new((186 * x) as f32, (107 * y * 2 + (if x % 2 == 0 {107} else {0})) as f32);
            shapes.push(
                meshes.add(Polyline2d::new(vec![
                    root_position + Vec2::new(62.0, -107.0),
                    root_position + Vec2::new(124.0, 0.0),
                    root_position + Vec2::new(62.0, 107.0),
                    root_position + Vec2::new(-62.0, 107.0),
                    root_position + Vec2::new(-124.0, 0.0),
                    root_position + Vec2::new(-62.0, -107.0)
                ]))
            );
        }
    }

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
            Transform::from_xyz(0.0, 0.0, 0.0)
        ));
    }
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut movement = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    let move_delta = movement.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.0);
    player.translation = player.translation.clamp(Vec3::new(-17670.0, -20330.0, -10.0), Vec3::new(17670.0, 20330.0, 10.0))
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_SPEED, time.delta_secs());
}
