use crate::{consts, errors::CompileError, XpCompiler};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct XpCallJson<'a> {
    language: &'a str,
    call: &'a str,
    args: Vec<&'a str>,
}

impl<'a> XpCallJson<'a> {
    pub fn new(language: &'a str, call: &'a str, args: Vec<&'a str>) -> Self {
        Self {
            language,
            call,
            args
        }
    }

    pub fn decode_call<G: XpCompiler>(self, compiler: G) -> Result<String, CompileError> {
        match self.call {
            consts::calls::CREATE_ACC => {
                if self.args.len() < 1 {
                    return Err(CompileError::InvalidArgs);
                }
                compiler.create_account(self.args[0]).map_err(|e| CompileError::from_generation(e, self.call.to_string()))
            },
            consts::calls::TRANSFER_AMOUNT => {
                if self.args.len() < 2 {
                    return Err(CompileError::InvalidArgs);
                }
                compiler.transfer_amount(self.args[0], self.args[1]).map_err(|e| CompileError::from_generation(e, self.call.to_string()))
            },
            _ => return Err(CompileError::UnsupportedCall(self.call.to_string())),
        }
    }

    pub fn compile(self) -> Result<String, CompileError> {
        match self.language {
            consts::langs::MOVE => {
                self.decode_call(move_compiler::generators::Generator)
            },
            consts::langs::SOLIDITY => {
                self.decode_call(solidity_compiler::generators::Generator)
            },
            _ => return Err(CompileError::UnsupportedLang(self.language.to_string()))
        }
    }
}
