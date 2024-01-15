// A division is an administrative district that is part of a larger division of a country.
// Divisions may, if they want, have their own subdivisions (Please don't do this, I beg of you, it's a nightmare).
// Divisions are assigned both a border representing their area, and a list of points representing the communities they administer.
// Upon creation, divisions should check to see if all of their child communities are within their borders.
// Divisions should verify that their borders are contiguous, and that they do not overlap with any other divisions that are not their parent or child.

use geo::{ BooleanOps, Contains };

pub struct Division {
    pub id: u64,
    pub name: String,
    pub parent: Option<Box<Division>>,
    pub area: geo::MultiPolygon<f64>,
    pub subdivisions: Vec<Box<Division>>,
}
impl Division {
    pub fn new(id: u64, name: String, parent: Option<Box<Division>>, area: geo::MultiPolygon<f64>) -> Division {
        Division {
            id,
            name,
            parent,
            area,
            subdivisions: Vec::new(),
        }
    }
    
    // This method loops through all of it's subdivisions
    // and their subdivisions, and so on, and returns creates a large multi-polygon representing
    // the total area of the division and all of it's subdivisions
    pub fn join_all(self) -> geo::MultiPolygon {
        let mut polygons = Vec::new();
        polygons.extend(self.area.0.clone());
        for subdivision in self.subdivisions {
            polygons.extend(subdivision.join_all().0.clone()); // Recursivity! :D
        }
        geo::MultiPolygon(polygons)
    }

    fn add_subdivision(&mut self, subdivision: Division) {
        self.subdivisions.push(Box::new(subdivision));
    }

    fn in_division(&self, point: &geo::Point<f64>) -> bool {
        // Check if the point is in the division's area
        self.area.contains(point)
    }

    fn in_division_poly(&self, polygon: &geo::Polygon<f64>) -> bool {
        // Check if the polygon is in the division's area
        self.area.contains(polygon)
    }


    // Function that takes a list of polygons, checks if they are all in the division, 
    // and after sufficiently checking, returns a new multi-polygon representing the area of the division
    fn create_area(&self, polys: &Vec<geo::Polygon<f64>>) -> Result<geo::MultiPolygon<f64>, ()> {
        // Check if all of the points are in the division
        for poly in polys {
            if !self.in_division_poly(poly) {
                return Err(());
            }
        }

        // Create a new multi-polygon from the points
        let mut polygons = Vec::new();
        for poly in polys {
            polygons.push(poly.clone());
        }
        Ok(geo::MultiPolygon(polygons))
    }



    pub fn create_subdivision(&mut self, name: String, polys: Vec<geo::Polygon>) -> Result<(), ()> {
        // Subtract the area of the subdivision from the area of the parent
        // If the parent has no area left, return an error
        // If the parent has area left, add the subdivision to the parent's list of subdivisions

        // Create a new division
        let mut subdivision = self.create_area(&polys)?;
        subdivision = self.area.difference(&subdivision);
        if subdivision.0.len() == 0 {
            return Err(());
        }
        let subdivision = Division::new(0, name, Some(Box::new(self.clone())), subdivision);
        Ok(())
    }
}
impl Clone for Division {
    fn clone(&self) -> Division {
        Division {
            id: self.id,
            name: self.name.clone(),
            parent: self.parent.clone(),
            area: self.area.clone(),
            subdivisions: self.subdivisions.clone(),
        }
    }
}