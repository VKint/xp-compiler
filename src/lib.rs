pub mod consts;
pub mod deserialize;
pub mod errors;

pub mod helpers;

#[cfg(test)]
mod tests;

use errors::GenerationError;

pub enum SupportedLanguages {
    Move,
    Solidity
}

pub trait XpCompiler {
    fn create_account(&self, address: &str) -> Result<String, GenerationError>;
    fn transfer_amount(&self, receiver: &str, amount: &str) -> Result<String, GenerationError>;
}

impl XpCompiler for move_compiler::generators::Generator {
    fn create_account(&self, address: &str) -> Result<String, GenerationError> {
        use move_compiler::generators::Generator;

        Ok(Generator::child_account_creation(
            consts::diem::COIN,
            address.parse().map_err(|_| GenerationError::ParseError)?,
            consts::diem::AUTH_PREFIX,
            false,
            0,
        ))
    }

    fn transfer_amount(&self, receiver: &str, amount: &str) -> Result<String, GenerationError> {
        use move_compiler::generators::Generator;

        Ok(Generator::payment_p2p(
                consts::diem::COIN,
                receiver.parse().map_err(|_| GenerationError::ParseError)?,
                amount.parse().map_err(|_| GenerationError::ParseError)?,
                None,
                None
        ))
    }
}

impl XpCompiler for solidity_compiler::generators::Generator {
    fn create_account(&self, _: &str) -> Result<String, GenerationError> {
        Err(GenerationError::UnsupportedCall)
    }

    fn transfer_amount(&self, receiver: &str, amount: &str) -> Result<String, GenerationError> {
        use solidity_compiler::generators::Generator;
        Ok(Generator::payment_p2p(receiver, amount))
    }
}
