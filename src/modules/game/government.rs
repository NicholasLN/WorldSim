use std::collections::HashMap;
use std::ops::Add;

use async_std::sync::{Arc, RwLock};

use crate::modules::game::division::UniqueDivision;
use crate::ProtectedState;

use super::Division;

pub type UniqueGovernment = Arc<RwLock<Government>>;
type TerritoryList = HashMap<u64, UniqueDivision>;

pub struct Government {
    pub state: ProtectedState,
    pub name: String,
    pub territories: TerritoryList,
}

impl Government {
    pub fn new(state: ProtectedState) -> Government {
        Government {
            state,
            name: String::new(),
            territories: HashMap::new(),
        }
    }
    pub async fn add_territory(&mut self, division: UniqueDivision) {
        let division_id = division.read().await.id;
        self.territories.insert(division_id, division);
    }
    pub async fn remove_territory(&mut self, division: UniqueDivision) -> bool {
        let division_id = division.read().await.id;
        self.territories.remove(&division_id);
        if (self.territories.get(&division_id).is_none()) {
            return true;
        }
        return false;
    }

    /// When a nation integrates a piece of land rather than annexing it. This maintains all local governance.
    /// TODO: Handle citizens from other nations. They should lose their positions, but the roles will exist as normal.
    pub async fn integrate_territory(&mut self, division: UniqueDivision, mut integrating_from: UniqueGovernment) {
        integrating_from.write().await.remove_territory(division.clone()).await;
        self.add_territory(division).await;
    }

    /// This method is for when a nation completely annexes a piece of land, eroding it's local government.
    /// TODO: Handle player roles. Politicians who are elected to subdivision positions should lose said position.
    pub async fn annex_territory(&mut self, division: UniqueDivision, mut annexing_from: UniqueGovernment) {
        let self_ref = Arc::new(RwLock::new(self.clone())); // Create a reference to self

        annexing_from.write().await.remove_territory(division.clone()).await;
        let mut read_access = division.write().await;
        let new_unorganized = read_access.join_all().await;

        let new_division = Division::new(
            read_access.id,
            read_access.name.clone().add(" Territory"),
            None,
            Some(self_ref.clone()),
            new_unorganized,
        );

        self.territories.insert(read_access.id, Arc::new(RwLock::new(new_division)));
    }
}

impl Clone for Government {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            name: self.name.clone(),
            territories: self.territories.clone(),
        }
    }
}