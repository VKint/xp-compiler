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

#[test]
fn test_move_transfer_amount() {
    let transfer_amount_data = r#"
        {
            "language": "move",
            "call": "transfer_amount",
            "args": ["1", "32"]
    }"#;
    let call: Result<XpCallJson, serde_json::Error> = serde_json::from_str(transfer_amount_data);
    assert!(call.is_ok());
    let call = call.unwrap();

    let compile = call.compile();
    assert!(compile.is_ok());
    if cfg!(feature = "test_generated") {
        println!("{}", compile.unwrap());
    }
}
