// Ensure that Executable is Sync
pub trait Executable: Send + Sync {
    fn execute(&self);
}

pub struct PowerBehavior {
    behavior: Box<dyn Executable>,
}

impl PowerBehavior {
    pub fn new(behavior: Box<dyn Executable>) -> Self {
        Self { behavior }
    }

    pub fn execute(&self) {
        self.behavior.execute();
    }
}

// Example implementation of a custom behavior
pub struct DeclareWarBehavior {
    // Fields for DeclareWarBehavior
}

impl Executable for DeclareWarBehavior {
    fn execute(&self) {
        // Implementation for executing DeclareWarBehavior
    }
}