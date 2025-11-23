use bevy::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const CAMERA_SPEED: f32 = 4.0;

#[derive(Component)]
struct Player;

// https://stackoverflow.com/a/40778485
fn hex_pos(x: f32, y: f32) -> Vec2 {
    let r = 124.0;
    let w = r * 2.0;
    let h = 3.0_f32.sqrt() * r;

    let mut pos = Vec2::ZERO;

    let r2 = r / 2.0;
    let h2 = h / 2.0;
    let mut xx = (x / 2.0).floor();
    let yy = (y / 2.0).floor();
    let xpos = (xx / 3.0).floor();
    xx %= 6.0;
    if xx % 3.0 == 0.0 {
        let mut xa = (x % r2) / r2;
        let mut ya = (y % h2) / h2;
        if yy % 2.0 == 0.0 {
            ya = 1.0 - ya;
        }
        if xx == 3.0 {
            xa = 1.0 - xa;
        }
        if xa > ya {
            pos.x = xpos + (if xx == 3.0 {-1.0} else {0.0});
            pos.y = ((yy + 1.0) / 2.0).floor();
            return pos;
        }
        pos.x = xpos + (if xx == 0.0 {-1.0} else {0.0});
        pos.y = ((yy + 1.0) / 2.0).floor();
        return pos;
    }
    if xx < 3.0 {
        pos.x = xpos + (if xx == 3.0 {-1.0} else {0.0});
        pos.y = (yy / 2.0).floor();
        return pos;
    }
    pos.x = xpos + (if xx == 0.0 {-1.0} else {0.0});
    pos.y = ((yy + 1.0) / 2.0).floor();
    pos
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, ((move_player, update_camera).chain(), mouse_interaction));
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
            let root_position = Vec2::new((186 * x) as f32, (y * 214 + (if x % 2 == 0 {107} else {0})) as f32);
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

fn mouse_interaction(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        && let Ok(viewport_check) = camera.world_to_viewport(camera_transform, world_pos.extend(0.0))
        && let Ok(world_check) = camera.viewport_to_world_2d(camera_transform, viewport_check.xy())
    {
        let pos = hex_pos(world_check.x + 18600.0, world_check.y + 21400.0);
        println!("Hex Position: {pos}");
    }
}
