use geo::{Geometry, MultiPolygon};
use geozero::{geojson::GeoJson, ToGeo};

use crate::modules::game::World;

// Helper function for transformation
fn transform_geo<T: AsRef<str>>(source_geojson: T) -> Result<MultiPolygon<f64>, ()> {
    let geojson = GeoJson(source_geojson.as_ref());
    let geometry = geojson.to_geo().unwrap();

    // Each source file should have a GeometryCollection with one MultiPolygon each
    // Take it out of the GeometryCollection and convert it to a MultiPolygon with f64 coordinates
    match geometry {
        Geometry::GeometryCollection(geometry_collection) => {
            let mut multi_polygons = Vec::new();
            for geometry in geometry_collection {
                match geometry {
                    Geometry::MultiPolygon(multi_polygon) => {
                        multi_polygons.extend(multi_polygon.into_iter());
                    }
                    _ => {
                        println!("Error. GeometryCollection should only contain MultiPolygons.");
                        return Err(());
                    }
                }
            }
            Ok(MultiPolygon(multi_polygons))
        }
        _ => {
            println!("Error. GeoJSON should only contain a GeometryCollection.");
            Err(())
        }
    }
}

fn create_physical_world() -> Result<World, ()> {
    let mut world = World::new();

    let seas = transform_geo(include_str!("../assets/world/earth_sea.json"))?;
    world.add_water_layer(seas);

    Ok(world)
}

pub fn create_world() -> Result<World, ()> {
    let world = create_physical_world()?;
    Ok(world)
}