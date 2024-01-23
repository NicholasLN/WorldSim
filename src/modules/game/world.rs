use geo::Contains;

use crate::modules::game::government::{Government, UniqueGovernment};

pub struct World {
    // Physical features
    pub water: geo::MultiPolygon<f64>,
    pub governments: Vec<UniqueGovernment>,
}

impl World {
    pub fn new() -> World {
        World {
            water: geo_types::MultiPolygon(vec![]),
            governments: Vec::new(),
        }
    }
    pub fn add_water_layer(&mut self, water: geo::MultiPolygon<f64>) {
        self.water = water;
    }


    pub fn on_land(&self, point: &geo::Point<f64>) -> bool {
        self.water.contains(point)
    }

    pub fn on_land_bulk(&self, points: &Vec<geo::Point<f64>>) -> Vec<bool> {
        let mut results = Vec::new();
        for point in points {
            results.push(self.on_land(point));
        }
        results
    }
}
impl Contains<geo::Point<f64>> for World {
    fn contains(&self, point: &geo::Point<f64>) -> bool {
        self.on_land(point)
    }
}
impl Contains<geo::MultiPolygon<f64>> for World {
    fn contains(&self, polygon: &geo::MultiPolygon<f64>) -> bool {
        self.water.contains(polygon)
    }
}
impl Clone for World {
    fn clone(&self) -> Self {
        Self {
            water: self.water.clone(),
            governments: self.governments.clone(),
        }
    }
}