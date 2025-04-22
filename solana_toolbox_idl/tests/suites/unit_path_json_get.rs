use serde_json::json;
use serde_json::Value;
use solana_toolbox_idl::ToolboxIdlPath;

#[tokio::test]
pub async fn run() {
    // Dummy JSON object pieces
    let content_object1 = json!({
        "object2": {},
        "array2": [],
        "number2": 20,
    });
    let content_array1 = json!([{}, [], 21]);
    let content = json!({
        "object1": content_object1,
        "array1": content_array1,
        "number1": 10,
    });
    // Check object reading
    assert_get(&content, "", json!(content));
    assert_get(&content, "object1", json!(content_object1));
    assert_get(&content, "object1.object2", json!({}));
    assert_get(&content, "object1.array2", json!([]));
    assert_get(&content, "object1.number2", json!(20));
    assert_get(&content, "array1", json!(content_array1));
    assert_get(&content, "array1.0", json!({}));
    assert_get(&content, "array1.1", json!([]));
    assert_get(&content, "array1.2", json!(21));
    assert_get(&content, "number1", json!(10));
}

fn assert_get(value: &Value, path: &str, expected: Value) {
    assert_eq!(
        ToolboxIdlPath::try_parse(path)
            .unwrap()
            .try_get_json_value(value)
            .unwrap(),
        &expected
    );
}
