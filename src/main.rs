use bevy::prelude::*;
use std::cmp::PartialEq;
use std::fmt::Display;

const PLAYER_SPEED: f32 = 250.0;
const CAMERA_SPEED: f32 = 4.0;

#[derive(Component)]
struct PositionText;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Hex;

#[derive(Component, Copy, Clone, PartialEq)]
struct HexCoord(IVec2);

impl Display for HexCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

fn hex_pos(pos: Vec2) -> HexCoord {
    let mut col = (pos.x / 186.0).round();
    let mut row = ((pos.y - if col % 2.0 == 0.0 { 107.0 } else { 0.0 }) / 214.0).round();

    if col == -0.0 {
        col = 0.0;
    }
    if row == -0.0 {
        row = 0.0;
    }

    HexCoord(IVec2::new(col as i32, row as i32))
}

fn spawn_hex(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    coord: IVec2,
) {
    let fill_mesh = meshes.add(
        ConvexPolygon::new(vec![
            Vec2::new(62.0, -107.0),
            Vec2::new(124.0, 0.0),
            Vec2::new(62.0, 107.0),
            Vec2::new(-62.0, 107.0),
            Vec2::new(-124.0, 0.0),
            Vec2::new(-62.0, -107.0),
        ])
        .unwrap()
        .mesh(),
    );

    let fill_mat = materials.add(Color::srgba(1.0, 1.0, 1.0, 0.0));

    let stroke_mesh = meshes.add(Polyline2d::new(vec![
        Vec2::new(62.0, -107.0),
        Vec2::new(124.0, 0.0),
        Vec2::new(62.0, 107.0),
        Vec2::new(-62.0, 107.0),
        Vec2::new(-124.0, 0.0),
        Vec2::new(-62.0, -107.0),
    ]));

    let stroke_mat = materials.add(Color::srgb(1.0, 0.5, 0.0));

    commands
        .spawn((
            Transform::from_xyz(position.x, position.y, 0.0),
            Hex,
            Visibility::default(),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh2d(fill_mesh),
                MeshMaterial2d(fill_mat),
                HexCoord(coord),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            p.spawn((
                Mesh2d(stroke_mesh),
                MeshMaterial2d(stroke_mat),
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        });
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        ((move_player, update_camera).chain(), mouse_interaction),
    );
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((Player, Transform::from_xyz(0.0, 0.0, 2.0)));

    commands.spawn((Text::new("(0,0)"), PositionText));

    for x in -100..100 {
        for y in -100..100 {
            let root_position = Vec2::new(
                (186 * x) as f32,
                (y * 214 + (if x % 2 == 0 { 107 } else { 0 })) as f32,
            );

            spawn_hex(
                &mut commands,
                &mut meshes,
                &mut materials,
                root_position,
                IVec2::new(x, y),
            );
        }
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
    player.translation = player.translation.clamp(
        Vec3::new(-17670.0, -20330.0, -10.0),
        Vec3::new(17670.0, 20330.0, 10.0),
    )
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
    mut position_text: Query<&mut Text, With<PositionText>>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tiles: Query<(&HexCoord, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        && let Ok(viewport_check) =
            camera.world_to_viewport(camera_transform, world_pos.extend(0.0))
        && let Ok(world_check) = camera.viewport_to_world_2d(camera_transform, viewport_check.xy())
    {
        let pos = hex_pos(world_check);
        for mut span in &mut position_text {
            **span = format!("{pos}");
        }

        for (coord, mut material_handle) in &mut tiles {
            let mat = materials.get_mut(&material_handle.0).unwrap();

            if coord == &pos {
                mat.color = Color::srgba(1.0, 1.0, 1.0, 0.1);
            } else {
                mat.color = Color::srgba(1.0, 1.0, 1.0, 0.0);
            }
        }
    }
}
