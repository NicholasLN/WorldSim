// A community is a group of people who live in a defined area.
// Communities contain the following information: 
// 1) Their administrative division (if any, otherwise they could be a city-state or something, 
//                                   let's just keep the option open for future expansion)
// 2) Their centerpoint (as a geo point)
// 3) Their boundaries. However, boundaries should not be necessary--they are just a list of geographic points for display purposes.
// 4) Population. For now, just a number. In the future, this will be more complex.
// 5) A list of facilities. These are the places where people work. They can be factories, offices, mines, etc.
// 6) Housing stock. How many homes are there? How many homes are vacant? How many homes are for sale? How many homes are for rent?
//      purely for statistical purposes for the simulation.
// 7) Urban resources. Does this city produce any special resources? Is it a port? Is it a mining town? Is it a tech hub?
//                     Resources may exist as their own point and node on the map, but they can also be tied to a community.
use geo::Point;
use super::Division;

pub struct Community {
    name: String,                    // This is the name of the community.
    division: Option<Box<Division>>, // This is the administrative division that the community is a part of.
    centerpoint: Point<f64>,         // This is the centerpoint of the community.
    boundaries: Vec<Point<f64>>,     // This is the list of points that make up the boundaries of the community.
    population: u32,                 // This is the population of the community.
}

impl Community {
    pub fn new(name: String, division: Option<Box<Division>>, centerpoint: Point<f64>, boundaries: Vec<Point<f64>>) -> Community {
        Community {
            name,
            division,
            centerpoint,
            boundaries,
            population: 0,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_division(&self) -> &Option<Box<Division>> {
        &self.division
    }
    pub fn get_centerpoint(&self) -> &Point<f64> {
        &self.centerpoint
    }
    pub fn get_boundaries(&self) -> &Vec<Point<f64>> {
        &self.boundaries
    }
    pub fn get_population(&self) -> &u32 {
        &self.population
    }
}