use galileo::{ MapBuilder, TileSchema };
use galileo::layer::FeatureLayer;
use galileo::symbol::CirclePointSymbol;
use galileo::galileo_types::latlon;
use galileo_types::geo::Crs;
use galileo::Color;

fn main() {
    MapBuilder::new()
        .center(latlon!(37.566, 126.9784))
        .resolution(TileSchema::web(18).lod_resolution(8).unwrap())
        .with_raster_tiles(|index| {
            format!("https://tile.openstreetmap.org/{}/{}/{}.png", index.z, index.x, index.y)
        },
        TileSchema::web(18))
        .with_layer(
            FeatureLayer::new(
                vec![latlon!(37.566, 126.9784)],
                CirclePointSymbol::new(Color::BLUE, 5.0),
                Crs::WGS84
            )
        )
        .build().await
        .run();
}
