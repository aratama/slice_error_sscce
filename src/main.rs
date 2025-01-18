use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 0.1;
    commands.spawn((Camera2d, projection));
    commands.spawn(AseSpriteSlice {
        aseprite: asset_server.load("test.aseprite"),
        name: "slice_red".into(),
    });
}

fn update(mut commands: Commands, mut query: Query<(Entity, &mut AseSpriteSlice)>) {
    for (entity, mut slice) in query.iter_mut() {
        slice.name = "slice_blue".into();
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(AsepriteUltraPlugin);
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
    app.run();
}
