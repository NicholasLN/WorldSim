use async_recursion::async_recursion;
use async_std::sync::{Arc, RwLock};
use geo::{BooleanOps, Contains, MultiPolygon, Point, Polygon};

use crate::modules::game::government::UniqueGovernment;
use crate::modules::game::role::RequiredRole;

pub type UniqueDivision = Arc<RwLock<Division>>;
pub type DivisionBoundaries = MultiPolygon<f64>;

pub struct Division {
    pub id: u64,
    pub name: String,
    pub parent: Option<UniqueDivision>,
    pub owner: Option<UniqueGovernment>,
    pub area: DivisionBoundaries,
    pub subdivisions: Vec<UniqueDivision>,
    pub required_positions: Vec<RequiredRole>,
}

impl Division {
    pub fn new(id: u64, name: String, parent: Option<UniqueDivision>, owner: Option<UniqueGovernment>, area: DivisionBoundaries) -> Self {
        Self {
            id,
            name,
            parent,
            owner,
            area,
            subdivisions: Vec::new(),
            required_positions: Vec::new(),
        }
    }

    #[async_recursion]
    pub async fn join_all(&self) -> MultiPolygon<f64> {
        let mut polygons = Vec::new();
        let self_area = self.area.clone();
        polygons.extend(self_area.into_iter());

        for subdivision in &self.subdivisions {
            let subdivision = subdivision.read().await;
            polygons.extend(subdivision.join_all().await.into_iter());
        }

        MultiPolygon(polygons)
    }

    pub async fn add_subdivision(&mut self, subdivision: UniqueDivision) {
        subdivision.write().await.owner = self.owner.clone();
        self.subdivisions.push(subdivision);
    }

    fn in_division(&self, point: &Point<f64>) -> bool {
        self.area.contains(point)
    }

    fn in_division_poly(&self, polygon: &Polygon<f64>) -> bool {
        self.area.contains(polygon)
    }

    fn create_area(&self, polys: &[Polygon<f64>]) -> Result<MultiPolygon<f64>, String> {
        if polys.iter().any(|poly| !self.in_division_poly(poly)) {
            return Err("Some polygons are not within the division's area".to_string());
        }
        Ok(MultiPolygon(polys.to_vec()))
    }

    fn create_area_with_points(&self, points: &[Point<f64>]) -> Result<MultiPolygon<f64>, String> {
        if points.iter().any(|point| !self.in_division(point)) {
            return Err("Some points are not within the division's area".to_string());
        }
        let polygon = Polygon::new(points.to_vec().into(), vec![]);
        if !self.in_division_poly(&polygon) {
            return Err("The created polygon is not within the division's area".to_string());
        }
        Ok(MultiPolygon(vec![polygon]))
    }


    pub async fn create_subdivision(&mut self, name: String, polys: Vec<Polygon<f64>>) -> Result<(), String> {
        let new_area = self.create_area(&polys)?;
        let difference_area = self.area.difference(&new_area);

        if difference_area.0.is_empty() {
            return Err("No area left for the parent division after subdivision".to_string());
        }

        let subdivision = Division::new(0, name, Some(Arc::new(RwLock::new(self.clone()))), self.owner.clone(), new_area);
        let subdivision = Arc::new(RwLock::new(subdivision));
        self.add_subdivision(subdivision).await;

        Ok(())
    }
}

impl Clone for Division {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            parent: self.parent.as_ref().map(|parent| Arc::clone(parent)),
            owner: self.owner.clone(),
            area: self.area.clone(),
            subdivisions: self.subdivisions.iter().map(|sd| Arc::clone(sd)).collect(),
            required_positions: self.required_positions.clone(),
        }
    }
}
