pub mod consts;
pub mod deserialize;
pub mod errors;

#[cfg(test)]
mod tests;

use errors::GenerationError;

pub enum SupportedLanguages {
    Move,
    Solidity,
}

// Common interface
pub trait XpCompiler {

    fn create_account(&self, address: &str) -> Result<String, GenerationError>;
    fn transfer_amount(&self, receiver: &str, amount: &str) -> Result<String, GenerationError>;
}

// Implementation for the move_compiler
impl XpCompiler for move_compiler::generators::Generator {

    /// Create a child VASP account
    ///
    /// address: u128
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
            None,
        ))
    }
}
// Implementation for the solidity_compiler
impl XpCompiler for solidity_compiler::generators::Generator {
    /// Not applicable in EVM
    fn create_account(&self, _: &str) -> Result<String, GenerationError> {
        Err(GenerationError::UnsupportedCall)
    }

    /// Transfer from one EVM address to another
    ///
    /// receiver: H160 \
    /// amount: U256
    fn transfer_amount(&self, receiver: &str, amount: &str) -> Result<String, GenerationError> {
        use solidity_compiler::generators::Generator;
        let amount = bigint::U256::from_dec_str(amount).map_err(|_| GenerationError::ParseError)?;
        let mut am_data: [u8; 32] = [0u8; 32];
        amount.to_big_endian(&mut am_data);
        let mut flag = false;
        let mut filtered: Vec<u8> = Vec::new();
        for e in am_data.iter() {
            if flag {
                filtered.push(*e);
            } else if *e != 0 {
                filtered.push(*e);
                flag = true;
            }
        }

        Ok(hex::encode(Generator::payment_p2p_bytes(
            &hex::decode(receiver).map_err(|_| GenerationError::ParseError)?,
            &filtered,
        )))
    }
}
