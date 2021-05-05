use move_binary_format::file_format::*;
use evm_asm::MoveCode;
use xp_compiler::deserialize::XpCallJson;

fn main() {
    // Reading the bytecode from Move
    let bytedata = include_bytes!("../../assets/call.mv");

    // The bytecode is deserialized to opcodes
    let script = CompiledScript::deserialize(bytedata).unwrap();

    // Wrapper around the opcodes
    let code = MoveCode::new_no_mods(script);

    // Converts it to our interface call
    let call = XpCallJson::from_move(&code).unwrap();

    // Compiles Solidity code
    let compile = call.compile().unwrap();
    println!("{}", compile);
}
