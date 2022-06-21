#[cfg(test)]
use odra::instance::Instance;
#[cfg(test)]
use odra::odra_types::bytesrepr::FromBytes;
#[cfg(test)]
use odra::odra_types::bytesrepr::ToBytes;
#[cfg(test)]
use odra::odra_types::{Address, RuntimeArgs};
use odra::Variable;

#[cfg(test)]
use odra::test_env::ContractContainer;
#[cfg(test)]
use odra::test_env::TestEnv;

#[odra::instance]
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

    pub fn forty_two(&self) -> u32 {
        42
    }
}

#[cfg(test)]
pub struct FlipperRef {
    pub address: Address,
}

#[cfg(test)]
impl FlipperRef {
    pub fn flip(&self) {
        TestEnv::call_contract(&self.address, "flip", &RuntimeArgs::new());
    }

    pub fn get(&self) -> bool {
        let raw_result = TestEnv::call_contract(&self.address, "get", &RuntimeArgs::new());
        <bool as FromBytes>::from_vec(raw_result).unwrap().0
    }

    pub fn cross_flip(&self, addr: Address) {
        TestEnv::call_contract(&addr, "flip", &RuntimeArgs::new());
    }

    pub fn forty_two(&self) -> u32 {
        let raw_result = TestEnv::call_contract(&self.address, "forty_two", &RuntimeArgs::new());
        <u32 as FromBytes>::from_vec(raw_result).unwrap().0
    }
}

#[cfg(test)]
impl Flipper {
    fn deploy(name: &str) -> FlipperRef {
        let mut container = ContractContainer {
            name: name.to_string(),
            wasm_path: "flipper.wasm".to_string(),
            entrypoints: Default::default(),
        };
        container.add("flip".to_string(), |name, _| {
            Flipper::instance(name.as_str()).flip();
            vec![]
        });

        container.add("get".to_string(), |name, _| {
            Flipper::instance(name.as_str()).get().to_bytes().unwrap()
        });

        let address = TestEnv::register_contract(&container);
        FlipperRef { address }
    }
}

#[cfg(test)]
mod tests {
    use crate::flipper::Flipper;

    #[test]
    fn flipping() {
        let contract = Flipper::deploy("flipper1");
        assert_eq!(contract.get(), false);
        contract.flip();
        assert_eq!(contract.get(), true);
    }

    #[test]
    fn test_two_flippers() {
        let contract1 = Flipper::deploy("flipper1");
        let contract2 = Flipper::deploy("flipper2");
        assert_eq!(contract1.get(), false);
        assert_eq!(contract2.get(), false);
        contract1.flip();
        assert_eq!(contract1.get(), true);
        assert_eq!(contract2.get(), false);
    }

    #[test]
    fn test_cross_flip() {
        let contract1 = Flipper::deploy("flipper1");
        let contract2 = Flipper::deploy("flipper2");
        assert_eq!(contract1.get(), false);
        assert_eq!(contract2.get(), false);
        contract1.cross_flip(contract2.address.clone());
        assert_eq!(contract1.get(), false);
        assert_eq!(contract2.get(), true);
        contract2.cross_flip(contract1.address.clone());
        assert_eq!(contract1.get(), true);
        assert_eq!(contract2.get(), true);
    }
}