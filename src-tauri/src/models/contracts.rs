pub struct Contract {
    duration: i8,
}

impl Contract {
    pub fn new(contract_duration: i8) -> Self {
        Contract {
            duration: contract_duration,
        }
    }
}
