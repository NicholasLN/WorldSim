use std::collections::HashMap;

use async_std::sync::{Arc, Mutex};

use crate::modules::game::power::Power;

/// `RequiredRole` is a type alias for an `Arc<Mutex<Role>>`. This construct is used to manage
/// roles within a larger entity, such as a government or organization. It ensures thread safety
/// and shared ownership of the `Role` data.
///
/// The `Arc` (Atomic Reference Counting) pointer allows multiple owners of the same data, which
/// is crucial in contexts where a role is accessed or modified by multiple threads or components.
///
/// The `Mutex` provides mutual exclusion, allowing only one thread to access the role data at a
/// time, ensuring data consistency and preventing race conditions.
pub type RequiredRole = Arc<Mutex<Role>>;

/// `UniqueRole` is a type alias similar to `RequiredRole`, also represented as an `Arc<Mutex<Role>>`.
/// This type is utilized for exclusive, thread-safe access to a unique `Role` within the system.
///
/// This setup is particularly useful in scenarios where a specific role, like a unique position
/// within a government or organization, needs to be accessed and modified by various parts of the
/// system concurrently, while maintaining data integrity.
pub type UniqueRole = Arc<Mutex<Role>>;

/// `Role` is a struct representing a position or role within an organization or government structure.
/// It contains an identifier, a name, and a list of powers associated with the role.
///
/// Fields:
/// - `id`: A unique identifier for the role, represented as a `u8`.
/// - `name`: A `String` representing the name of the role (e.g., "President", "Senator").
/// - `powers`: A `Vec<Power>` holding the various powers or authorities that the role possesses.
///
/// The `Role` struct is central to the management of positions within a system, allowing for
/// the definition and assignment of specific capabilities and responsibilities to different roles.
pub struct Role {
    pub id: u8,
    pub name: String,
    pub powers: Vec<Power>,
}

pub struct RoleCollection {
    roles: HashMap<u8, UniqueRole>,
}

impl RoleCollection {
    pub fn new() -> Self {
        Self {
            roles: HashMap::new()
        }
    }
    pub fn append_raw_role(&mut self, role: Role) {
        self.roles.insert(role.id, Arc::new(Mutex::new(role)));
    }
    pub async fn append_role(&mut self, role: UniqueRole) {
        let role_id = role.lock().await.id;
        self.roles.insert(role_id, role);
    }
    pub fn get_role(&self, id: u8) -> Option<UniqueRole> {
        self.roles.get(&id).cloned()
    }
    pub fn delete_role(&mut self, id: u8) -> bool {
        self.roles.remove(&id);
        if self.get_role(id).is_some() {
            return true;
        }
        return false;
    }
}