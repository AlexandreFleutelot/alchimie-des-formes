use bevy::prelude::*;
use super::logic::grid::Quarter;
use super::logic::colors::QColor;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        println!("RenderPlugin build");
        app.add_systems(Startup, setup);
    }
}

fn quarter_triangle(q: Quarter) -> Triangle2d {
    let mid = Vec2::new(50.,50.);
    match q {
        Quarter::Nord => Triangle2d::new(Vec2::new(0., 100.), Vec2::new(100., 100.), mid),
        Quarter::Est => Triangle2d::new(Vec2::new(100., 100.), Vec2::new(100., 0.), mid),
        Quarter::Sud => Triangle2d::new(Vec2::new(100., 0.), Vec2::new(0., 0.), mid),
        Quarter::Ouest => Triangle2d::new(Vec2::new(0., 0.), Vec2::new(0., 100.), mid),
    }
}

fn to_color(c: QColor) -> Option<Color> {
    match c {
        QColor::Empty => None,
        QColor::Rouge => Some(Color::srgb(1., 0., 0.)),
        QColor::Jaune => Some(Color::srgb(1., 1., 0.)),
        QColor::Bleu => Some(Color::srgb(0., 0., 1.)),
        QColor::Orange => Some(Color::srgb(1., 0.5, 0.)),
        QColor::Vert => Some(Color::srgb(0., 1., 0.)),
        QColor::Violet => Some(Color::srgb(1., 0., 1.)),
        QColor::Brun => Some(Color::srgb(0.45, 0.3, 0.15)),
    }
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    let qnorth = quarter_triangle(Quarter::Nord);
    let qeast = quarter_triangle(Quarter::Est);
    let qsouth = quarter_triangle(Quarter::Sud);
    let qwest = quarter_triangle(Quarter::Ouest);

    let red = MeshMaterial2d(materials.add(Color::srgb(1., 0., 0.)));
    let green = MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.)));
    let blue = MeshMaterial2d(materials.add(Color::srgb(0., 0., 1.)));
    let purple = MeshMaterial2d(materials.add(Color::srgb(1., 0., 1.)));

    commands.spawn((Mesh2d(meshes.add(qnorth)), red));
    commands.spawn((Mesh2d(meshes.add(qeast)), blue));
    commands.spawn((Mesh2d(meshes.add(qsouth)), green));
    commands.spawn((Mesh2d(meshes.add(qwest)), purple));

}

