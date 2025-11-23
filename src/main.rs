use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

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