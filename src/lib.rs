mod consts;
pub mod deserialize;
mod errors;

#[cfg(test)]
mod tests;

pub enum SupportedLanguages {
    Move
}

pub trait XpCompiler {
    fn create_account(&self, address: u128) -> String;
    fn transfer_amount(&self, receiver: u128, amount: u64) -> String;
}

impl XpCompiler for move_compiler::generators::Generator {
    fn create_account(&self, address: u128) -> String {
        use move_compiler::generators::Generator;

        Generator::child_account_creation(consts::diem::COIN, address, consts::diem::AUTH_PREFIX, false, 0)
    }

    fn transfer_amount(&self, receiver: u128, amount: u64) -> String {
        use move_compiler::generators::Generator;

        Generator::payment_p2p(consts::diem::COIN,receiver, amount, None, None)
    }
}
