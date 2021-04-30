use crate::{consts, errors::CompileError, XpCompiler};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct XpCallJson<'a> {
    language: &'a str,
    call: &'a str,
    args: Vec<&'a str>,
}

impl XpCallJson<'_> {
    pub fn compile(self) -> Result<String, CompileError> {
        let compiler = match self.language {
            consts::langs::MOVE => move_compiler::generators::Generator,
            _ => return Err(CompileError::UnsupportedLang(self.language.to_string())),
        };

        match self.call {
            consts::calls::CREATE_ACC => {
                if self.args.len() < 1 {
                    return Err(CompileError::InvalidArgs);
                }
                let address: u128 = self.args[0]
                    .parse()
                    .map_err(|_| CompileError::InvalidArgs)?;
                Ok(compiler.create_account(address))
            }
            consts::calls::TRANSFER_AMOUNT => {
                if self.args.len() < 2 {
                    return Err(CompileError::InvalidArgs);
                }
                let receiever: u128 = self.args[0]
                    .parse()
                    .map_err(|_| CompileError::InvalidArgs)?;
                let amount: u64 = self.args[1]
                    .parse()
                    .map_err(|_| CompileError::InvalidArgs)?;
                Ok(compiler.transfer_amount(receiever, amount))
            }
            _ => return Err(CompileError::UnsupportedCall(self.call.to_string())),
        }
    }
}
