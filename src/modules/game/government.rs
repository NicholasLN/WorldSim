use super::Division;

pub struct Government {
    pub name: String,
    pub territories: Vec<Box<Division>>,
}
impl Government {
    pub fn new() -> Government {
        Government {
            name: String::new(),
            territories: Vec::new(),
        }
    }
    pub fn add_territory(&mut self, division: Division) {
        self.territories.push(Box::new(division)); 
        // Mostly during initialization of world.
    }

    pub fn remove_territory(&mut self, division: Division) {
        let mut index = 0;
        for territory in &self.territories {
            if (territory.id == division.id && territory.name == division.name) {
                break;
            }
            index += 1;
        }
        self.territories.remove(index);
    }

    pub fn integrate_territory(&mut self, division: Division, mut integrating_from: Government) {
        integrating_from.remove_territory(division.clone());
        self.add_territory(division);
    }
    
    pub fn annex_territory(&mut self, division: Division, mut annexing_from: Government) {
        annexing_from.remove_territory(division.clone());
        let new_unorganized: geo::MultiPolygon = division.join_all();
        let new_division = Division::new(0, format!("${} Unorganized", self.name), None, new_unorganized);   
        // Add the new division to the list of territories
        self.territories.push(Box::new(new_division));         
    }
}