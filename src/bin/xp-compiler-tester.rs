use move_binary_format::file_format::*;
use evm_asm::MoveCode;
use xp_compiler::deserialize::XpCallJson;

fn main() {
    let bytedata = include_bytes!("../../assets/call.mv");

    let script = CompiledScript::deserialize(bytedata).unwrap();
    let code = MoveCode::new_no_mods(script);

    // Convert it to our interface call
    let call = XpCallJson::from_move(&code).unwrap();

    // Compile solidity code
    let compile = call.compile().unwrap();
    println!("{}", compile);
}
