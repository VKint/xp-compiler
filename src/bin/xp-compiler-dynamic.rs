// See assets/coin_disasm.txt for psuedo-opcode of coin.mv
// See assets/coin_sol.txt for resultant evm opcode including its explanation

use evm_asm::MoveCode;
use rsevmasm::{Instruction};
use move_binary_format::file_format::*;

enum MapRes {
    Done,
    Call(FunctionHandleIndex)
}

// Warn: This is not very efficient
// This doesn't use a stack based calling convention, because Move doesn't either
// WARN: assumes each call takes only 1 argument
fn match_ins_raw(ins: &Bytecode, res: &mut Vec<Instruction>) -> MapRes {
    match ins {
        Bytecode::LdU64(val) => {
            let lz = val.leading_zeros() as usize/8;
            res.push(Instruction::Push(val.to_be_bytes()[lz..8].to_vec()));
        },
        Bytecode::Call(idx) => {
            res.push(Instruction::Push(vec![0]));
            res.push(Instruction::MStore);
            return MapRes::Call(*idx);
        },
        Bytecode::MoveLoc(idx) | Bytecode::CopyLoc(idx) => {
            res.push(Instruction::Push(vec![*idx as u8]));
            res.push(Instruction::MLoad);
        }
        Bytecode::StLoc(idx) => {
            res.push(Instruction::Push(vec![*idx as u8]));
            res.push(Instruction::MStore);
        },
        Bytecode::Pop => {
            res.push(Instruction::Pop)
        },
        Bytecode::Pack(_) => {
            res.push(Instruction::Push(vec![0]));
            res.push(Instruction::MLoad);
        },
        Bytecode::Unpack(_) => (), // Structs are stored untyped on the stack
        Bytecode::Ret => (),
        _ => unimplemented!()
    }

    return MapRes::Done;
}

fn main() {
    // Example coin script
    let coin_s = include_bytes!("../../assets/coin.mv");
    // Coin module
    let cmod = include_bytes!("../../assets/coin_m.mv");

    // Deserialize bytecode
    let script = CompiledScript::deserialize(coin_s).unwrap();
    let cmd = CompiledModule::deserialize(cmod).unwrap();

    let mv = MoveCode::new(script, vec![cmd]);
    let mut solc = vec![Instruction::Dup(0x1)];

    for instruction in &mv.script.code.code {
        if let MapRes::Call(idx) = match_ins_raw(&instruction, &mut solc) {
            let c = mv.resolve_call(idx).unwrap();
            for ins in &c.code {
                match_ins_raw(ins, &mut solc);
            }
        }
    }

    solc.push(Instruction::Stop);

    let raw = rsevmasm::assemble_instructions(solc);
    println!("Solidity Bytecode: {}", hex::encode(&raw));

    println!("Dissassembly");
    evm_asm::helpers::disassemble_evm(&raw).unwrap();
}
