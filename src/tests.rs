use crate::deserialize::XpCallJson;

#[test]
fn test_move_create_child() {
    let create_acc_data = r#"
    {
        "language": "move",
        "call": "create_account",
        "args": ["1"]
    }"#;
    let call: Result<XpCallJson, serde_json::Error> = serde_json::from_str(create_acc_data);
    assert!(call.is_ok());
    let call = call.unwrap();

    let compile = call.compile();
    assert!(compile.is_ok());
    if cfg!(feature = "test_generated") {
        println!("{}", compile.unwrap());
    }
}

fn test_transfer_amount(data: &str) {
    let call: Result<XpCallJson, serde_json::Error> = serde_json::from_str(data);
    assert!(call.is_ok());
    let call = call.unwrap();

    let compile = call.compile();
    assert!(compile.is_ok());
    if cfg!(feature = "test_generated") {
        println!("{}", compile.unwrap());
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
