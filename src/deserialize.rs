use crate::{consts, errors::CompileError, XpCompiler};
use evm_asm::MoveCode;
use move_binary_format::file_format::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct XpCallJson {
    language: String,
    call: String,
    args: Vec<String>,
}

impl XpCallJson {
    pub fn new(language: String, call: String, args: Vec<String>) -> Self {
        Self {
            language,
            call,
            args,
        }
    }

    pub fn from_move(code: &MoveCode) -> Result<Self, CompileError> {
        let mut lang: Option<String> = None;
        let mut cflag = false;
        let mut args = Vec::<String>::new();
        let mut call: Option<String> = None;
        for instruction in &code.script.code.code {
            match instruction {
                Bytecode::LdConst(idx) => {
                    let con = code.resolve_const(*idx);
                    let raw = code.const_to_vec8(con.clone());
                    if raw.is_none() {
                        continue;
                    }
                    let raw = raw.unwrap();
                    let s = String::from_utf8(raw);
                    if s.is_err() {
                        continue;
                    }
                    let s = s.unwrap();
                    if lang.is_none() {
                        lang = Some(s);
                    } else {
                        args.push(s);
                    }
                }
                Bytecode::Call(idx) if lang.is_some() => {
                    if cflag {
                        let fun = code.fn_handle(*idx).name;
                        call = Some(code.identifier_resolve(fun).as_str().to_string());
                        break;
                    } else {
                        cflag = true;
                    }
                }
                _ => (),
            }
        }

        Ok(Self {
            language: lang.ok_or(CompileError::InvalidArgs)?,
            call: call.ok_or(CompileError::InvalidArgs)?,
            args: args,
        })
    }

    pub fn decode_call<G: XpCompiler>(self, compiler: G) -> Result<String, CompileError> {
        match self.call.as_str() {
            consts::calls::CREATE_ACC => {
                if self.args.len() < 1 {
                    return Err(CompileError::InvalidArgs);
                }
                compiler
                    .create_account(&self.args[0])
                    .map_err(|e| CompileError::from_generation(e, self.call.to_string()))
            }
            consts::calls::TRANSFER_AMOUNT => {
                if self.args.len() < 2 {
                    return Err(CompileError::InvalidArgs);
                }
                compiler
                    .transfer_amount(&self.args[0], &self.args[1])
                    .map_err(|e| CompileError::from_generation(e, self.call.to_string()))
            }
            _ => return Err(CompileError::UnsupportedCall(self.call.to_string())),
        }
    }

    pub fn compile(self) -> Result<String, CompileError> {
        match self.language.as_str() {
            consts::langs::MOVE => self.decode_call(move_compiler::generators::Generator),
            consts::langs::SOLIDITY => self.decode_call(solidity_compiler::generators::Generator),
            _ => return Err(CompileError::UnsupportedLang(self.language.to_string())),
        }
    }
}
