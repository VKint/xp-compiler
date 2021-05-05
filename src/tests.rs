use crate::deserialize::XpCallJson;
use evm_asm::MoveCode;
use move_binary_format::file_format::*;

#[test]
fn test_move_create_child() {
    let create_acc_data = r#"
    {
        "language": "move",
        "call": "create_account",
        "args": ["1"]
    }"#;
    let call: XpCallJson = serde_json::from_str(create_acc_data).unwrap();

    let compile = call.compile().unwrap();
    if cfg!(feature = "test_generated") {
        println!("{}", compile);
    }
}

fn test_transfer_amount(data: &str) {
    let call: XpCallJson = serde_json::from_str(data).unwrap();

    let compile = call.compile().unwrap();
    if cfg!(feature = "test_generated") {
        println!("{}", compile);
    }
}

#[test]
fn test_move_transfer_amount() {
    let transfer_amount_data = r#"
        {
            "language": "move",
            "call": "transfer_amount",
            "args": ["1", "32"]
    }"#;
    test_transfer_amount(transfer_amount_data);
}

#[test]
fn test_solidity_transfer_amount() {
    let transfer_amount_data = r#"
    {
        "language": "solidity",
        "call": "transfer_amount",
        "args": ["106Ca83003090c63B03d3fE3A9EE3B5E36C155CD", "32"]
    }"#;
    test_transfer_amount(transfer_amount_data);
}

#[test]
fn test_move_deserialize() {
    // Import bytecode from pre-compiled move template
    let bytedata = include_bytes!("../assets/call.mv");
    // deserialize it to opcodes
    let script = CompiledScript::deserialize(bytedata).unwrap();
    let code = MoveCode::new_no_mods(script);

    // Convert it to our interface call
    let call = XpCallJson::from_move(&code).unwrap();

    // Compile solidity code
    let compile = call.compile().unwrap();
    if cfg!(feature = "test_generated") {
        println!("{}", compile);
    }
}
