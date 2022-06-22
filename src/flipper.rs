use odra::Variable;

#[odra::contract]
pub struct Flipper {
    value: Variable<bool>,
}

#[odra::contract]
impl Flipper {
    pub fn set(&self, value: bool) {
        self.value.set(value);
    }

    pub fn flip(&self) {
        if let Some(value) = self.value.get() {
            self.value.set(!value);
        } else {
            self.value.set(true);
        }
        self.value.set(true);
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
        let contract = Flipper::deploy("flipper1");
        assert!(!contract.get());
        contract.flip();
        assert!(contract.get());
    }

    #[test]
    fn test_two_flippers() {
        let contract1 = Flipper::deploy("flipper1");
        let contract2 = Flipper::deploy("flipper2");
        assert!(!contract1.get());
        assert!(!contract2.get());
        contract1.flip();
        assert!(contract1.get());
        assert!(!contract2.get());
    }
}
