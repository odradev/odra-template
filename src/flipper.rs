use odra::Variable;

/// A module definition. Each module struct consists Variables and Mappings
/// or/and another modules.
#[odra::module]
pub struct Flipper {
    /// The module itself does not store the value, 
    /// it's a proxy that writes/reads value to/from the host.
    value: Variable<bool>,
}

/// Module implementation.
/// 
/// To generate entrypoints,
/// an implementation block must be marked as #[odra::module].
#[odra::module]
impl Flipper {
    /// Odra constructor.
    /// 
    /// Initializes the contract with the value of value.
    #[odra(init)]
    pub fn initial_settings(&self) {
        self.value.set(false);
    }

    /// Replaces the current value with the passed argument.
    pub fn set(&self, value: bool) {
        self.value.set(value);
    }

    /// Replaces the current value with the opposite value.
    pub fn flip(&self) {
        self.value.set(!self.get());
    }

    /// Retrieves value from the storage. 
    /// If the value has never been set, the default value is returned.
    pub fn get(&self) -> bool {
        self.value.get_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::flipper::Flipper;

    #[test]
    fn flipping() {
        /// To test a module we need to deploy it. 
        /// To do so, Odra generates for us deploy() function.
        /// To call a constructor we don't do it directly. In this case to call
        /// a constructor, we would call deploy_initial_settings() function.
        let contract = Flipper::deploy();
        assert!(!contract.get());
        contract.flip();
        assert!(contract.get());
    }

    #[test]
    fn test_two_flippers() {
        let contract1 = Flipper::deploy();
        let contract2 = Flipper::deploy();
        assert!(!contract1.get());
        assert!(!contract2.get());
        contract1.flip();
        assert!(contract1.get());
        assert!(!contract2.get());
    }
}
