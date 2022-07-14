use odra::Variable;

#[odra::module]
pub struct Flipper {
    value: Variable<bool>,
}

#[odra::module]
impl Flipper {
    #[odra(init)]
    pub fn initial_settings(&self) {
        self.value.set(false);
    }

    pub fn set(&self, value: bool) {
        self.value.set(value);
    }

    pub fn flip(&self) {
        self.value.set(!self.get());
    }

    pub fn get(&self) -> bool {
        self.value.get_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::flipper::Flipper;

    #[test]
    fn flipping() {
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
