use serde_json::json;
use serde_json::Value;
use solana_toolbox_idl::ToolboxIdlPath;

#[tokio::test]
pub async fn run() {
    // Dummy JSON object pieces
    let original_object1 = json!({
        "object2": {},
        "array2": [],
    });
    let original_array1 = json!([{}, []]);
    let original = json!({
        "object1": original_object1,
        "array1": original_array1,
    });
    // Check root insert
    assert_path_set_to_42(
        Some(original.clone()),
        "new",
        json!({
            "object1": original_object1,
            "array1": original_array1,
            "new": 42,
        }),
    );
    // Check root replace
    assert_path_set_to_42(
        Some(original.clone()),
        "object1",
        json!({
            "object1": 42,
            "array1": original_array1,
        }),
    );
    // Check root.object1 insert
    assert_path_set_to_42(
        Some(original.clone()),
        "object1.new",
        json!({
            "object1": {
                "object2": {},
                "array2": [],
                "new": 42,
            },
            "array1": original_array1,
        }),
    );
    // Check root.object1 replace
    assert_path_set_to_42(
        Some(original.clone()),
        "object1.object2",
        json!({
            "object1": {
                "object2": 42,
                "array2": [],
            },
            "array1": original_array1,
        }),
    );
    // Check root.object1.object2 insert
    assert_path_set_to_42(
        Some(original.clone()),
        "object1.object2.new",
        json!({
            "object1": {
                "object2": { "new": 42 },
                "array2": [],
            },
            "array1": original_array1,
        }),
    );
    // Check root.object1.array2[0] replace
    assert_path_set_to_42(
        Some(original.clone()),
        "object1.array2.0",
        json!({
            "object1": {
                "object2": {},
                "array2": [42],
            },
            "array1": original_array1,
        }),
    );
    // Check root.array1[1] replace
    assert_path_set_to_42(
        Some(original.clone()),
        "array1.1",
        json!({
            "object1": original_object1,
            "array1": [
                {},
                42,
            ],
        }),
    );
    // Check root.array1[] append
    assert_path_set_to_42(
        Some(original.clone()),
        "array1.",
        json!({
            "object1": original_object1,
            "array1": [
                {},
                [],
                42,
            ],
        }),
    );
    // Check root.array1[0] insert
    assert_path_set_to_42(
        Some(original.clone()),
        "array1.0.new",
        json!({
            "object1": original_object1,
            "array1": [
                { "new": 42 },
                [],
            ],
        }),
    );
    // Check root.array1[1][0] insert
    assert_path_set_to_42(
        Some(original.clone()),
        "array1.1.0",
        json!({
            "object1": original_object1,
            "array1": [
                {},
                [42],
            ],
        }),
    );
    // Check specials
    assert_path_set_to_42(Some(json!({})), "", json!(42));
    assert_path_set_to_42(Some(json!({})), ".", json!({ "": 42 }));
    assert_path_set_to_42(Some(json!({})), "a", json!({ "a": 42 }));
    assert_path_set_to_42(Some(json!([])), "", json!(42));
    assert_path_set_to_42(Some(json!([])), ".", json!([42]));
    assert_path_set_to_42(Some(json!([])), "0", json!([42]));
    assert_path_set_to_42(None, "", json!(42));
    assert_path_set_to_42(None, ".", json!([42]));
    assert_path_set_to_42(None, "0", json!([42]));
    assert_path_set_to_42(None, "a", json!({ "a": 42 }));
    assert_path_set_to_42(None, "a.b.c", json!({ "a": { "b": { "c": 42 } } }));
    assert_path_set_to_42(None, "a..b.0", json!({ "a": [{ "b": [42] }] }));
}

fn assert_path_set_to_42(node: Option<Value>, path: &str, expected: Value) {
    assert_eq!(
        ToolboxIdlPath::try_parse(path)
            .unwrap()
            .try_set_json_value(node, json!(42))
            .unwrap(),
        expected
    );
}
