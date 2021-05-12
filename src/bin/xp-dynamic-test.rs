use evm_asm::MoveCode;
use move_binary_format::file_format::*;

fn main() {
    // Example coin script
    let coin_s = include_bytes!("../../assets/coin.mv");
    // Coin module
    let cmod = include_bytes!("../../assets/coin_m.mv");

    // Deserialize bytecode
    let script = CompiledScript::deserialize(coin_s).unwrap();
    let cmd = CompiledModule::deserialize(cmod).unwrap();

    let mv = MoveCode::new(script, vec![cmd]);

    let solc = evm_asm::helpers::move_recompile_to_evm(&mv);
    let raw = rsevmasm::assemble_instructions(solc);

    println!("{}", hex::encode(&raw));
}
