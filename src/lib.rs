use bevy::prelude::*;

use std::collections::HashMap;

#[derive(Component)]
struct Player;

#[derive(Component, Copy, Clone, PartialEq)]
struct Hex(IVec2);

impl std::fmt::Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[derive(Component)]
struct Tile;

#[derive(Resource, Clone, Copy)]
pub struct HexForgeConfig {
    pub player_speed: f32,
    pub camera_speed: f32,
    pub stroke_color: Color,
    pub fill_color: Color,
    pub hover_color: Color,
}

#[derive(Resource, Clone)]
struct HexForgeMemory {
    active_hexes: HashMap<IVec2, Entity>,
    hexes: Vec<IVec2>,
    current_pos: IVec2
}

pub struct HexForge {
    pub config: HexForgeConfig,
    memory: HexForgeMemory,
}

fn hex_pos(pos: Vec2) -> Hex {
    let mut col = (pos.x / 186.0).round();
    let mut row = ((pos.y - if col % 2.0 == 0.0 { 107.0 } else { 0.0 }) / 214.0).round();

    if col == -0.0 {
        col = 0.0;
    }
    if row == -0.0 {
        row = 0.0;
    }

    Hex(IVec2::new(col as i32, row as i32))
}

fn spawn_hex(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    coord: IVec2,
    config: &Res<HexForgeConfig>,
    memory: &mut ResMut<HexForgeMemory>,
) {
    let vertices = vec![
        Vec2::new(62.0, -107.0),
        Vec2::new(124.0, 0.0),
        Vec2::new(62.0, 107.0),
        Vec2::new(-62.0, 107.0),
        Vec2::new(-124.0, 0.0),
        Vec2::new(-62.0, -107.0),
    ];

    let fill_mesh = meshes.add(ConvexPolygon::new(vertices.clone()).unwrap().mesh());

    let fill_mat = materials.add(config.fill_color);

    let stroke_mesh = meshes.add(Polyline2d::new(vertices));

    let stroke_mat = materials.add(config.stroke_color);

    let entity = commands
        .spawn((
            Transform::from_xyz(position.x, position.y, 0.0),
            Visibility::default(),
            Tile
        ))
        .with_children(|p| {
            p.spawn((
                Mesh2d(fill_mesh),
                MeshMaterial2d(fill_mat),
                Hex(coord),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            p.spawn((
                Mesh2d(stroke_mesh),
                MeshMaterial2d(stroke_mat),
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        }).id();
    memory.active_hexes.insert(coord, entity);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    config: Res<HexForgeConfig>,
    mut memory: ResMut<HexForgeMemory>,
) {
    commands.spawn(Camera2d);

    commands.spawn((Player, Transform::from_xyz(0.0, 0.0, 2.0)));

    for x in -5..=5 {
        for y in -5..=5 {
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
                &config,
                &mut memory
            );
        }
    }
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    config: Res<HexForgeConfig>,
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

    let move_delta = movement.normalize_or_zero() * config.player_speed * time.delta_secs();
    player.translation += move_delta.extend(0.0);
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    config: Res<HexForgeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut memory: ResMut<HexForgeMemory>,
    mut commands: Commands,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, config.camera_speed, time.delta_secs());

    if hex_pos(camera.translation.xy()).0 != memory.current_pos {
        memory.current_pos = hex_pos(camera.translation.xy()).0;

        let mut to_be_removed: Vec<IVec2> = Vec::new();
        for (pos, entity) in memory.active_hexes.iter() {
            if pos.x < memory.current_pos.x - 5 || pos.x > memory.current_pos.x + 5 ||
                pos.y < memory.current_pos.y - 5 || pos.y > memory.current_pos.y + 5 {
                commands.entity(*entity).despawn_children().despawn();
                to_be_removed.push(*pos);
            }
        }

        for x in memory.current_pos.x - 5..=memory.current_pos.x+5 {
            for y in memory.current_pos.y - 5..=memory.current_pos.y + 5 {
                if !memory.active_hexes.contains_key(&IVec2::new(x, y)) {
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
                        &config,
                        &mut memory
                    );
                }
            }
        }

        for pos in to_be_removed {
            memory.active_hexes.remove(&pos);
        }
    }
}

fn mouse_interaction(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tiles: Query<(&Hex, &mut MeshMaterial2d<ColorMaterial>)>,

    config: Res<HexForgeConfig>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        && let Ok(viewport_check) =
            camera.world_to_viewport(camera_transform, world_pos.extend(0.0))
        && let Ok(world_check) = camera.viewport_to_world_2d(camera_transform, viewport_check.xy())
    {
        let pos = hex_pos(world_check);

        for (coord, material_handle) in &mut tiles {
            let mat = materials.get_mut(&material_handle.0).unwrap();

            if coord == &pos {
                mat.color = config.hover_color;
            } else {
                mat.color = config.fill_color;
            }
        }
    }
}

impl HexForge {
    pub fn new(config: HexForgeConfig) -> Self {
        Self {
            config,
            memory: HexForgeMemory {
                active_hexes: HashMap::new(),
                hexes: Vec::new(),
                current_pos: IVec2::ZERO
            },
        }
    }

    pub fn start(&mut self) {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        app.insert_resource(self.config);
        app.insert_resource(self.memory.clone());

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            ((move_player, update_camera).chain(), mouse_interaction),
        );
        app.run();
    }

    pub fn save(&mut self) -> Vec<IVec2> {
        self.memory.hexes.clone()
    }
}
