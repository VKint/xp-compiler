// See assets/coin_disasm.txt for psuedo-opcode of coin.mv
// See assets/coin_sol.txt for resultant evm opcode including its explanation

use evm_asm::MoveCode;
use move_binary_format::file_format::*;

fn main() {
    // Example coin script
    let coin_s = include_bytes!("../../assets/coin.mv");
    // Coin module
    let cmod = include_bytes!("../../assets/coin_m.mv");

    println!("Move Script Bytecode: {}", hex::encode(&coin_s));
    println!("Move Module bytecode: {}", hex::encode(&cmod));

    // Deserialize bytecode
    let script = CompiledScript::deserialize(coin_s).unwrap();
    let cmd = CompiledModule::deserialize(cmod).unwrap();

    let mv = MoveCode::new(script, vec![cmd]);

    println!();
    println!("Move Disassembly");
    mv.disassemble_with_mods().unwrap();
    println!();


    let solc = evm_asm::helpers::move_recompile_to_evm(&mv);
    let raw = rsevmasm::assemble_instructions(solc);


    println!("Solidity Disassembly");
    evm_asm::helpers::disassemble_evm(&raw).unwrap();

    println!("Soldity Bytecode: {}", hex::encode(&raw));
}
