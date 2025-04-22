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
    // Check root swap
    assert_set_to_null(Some(original.clone()), "", json!(null));
    // Check root insert
    assert_set_to_null(
        Some(original.clone()),
        "new",
        json!({
            "object1": original_object1,
            "array1": original_array1,
            "new": null,
        }),
    );
    // Check root replace
    assert_set_to_null(
        Some(original.clone()),
        "object1",
        json!({
            "object1": null,
            "array1": original_array1,
        }),
    );
    // Check root.object1 insert
    assert_set_to_null(
        Some(original.clone()),
        "object1.new",
        json!({
            "object1": {
                "object2": {},
                "array2": [],
                "new": null,
            },
            "array1": original_array1,
        }),
    );
    // Check root.object1 replace
    assert_set_to_null(
        Some(original.clone()),
        "object1.object2",
        json!({
            "object1": {
                "object2": null,
                "array2": [],
            },
            "array1": original_array1,
        }),
    );
    // Check root.object1.object2 insert
    assert_set_to_null(
        Some(original.clone()),
        "object1.object2.new",
        json!({
            "object1": {
                "object2": { "new": null },
                "array2": [],
            },
            "array1": original_array1,
        }),
    );
    // Check root.object1.array2[0] replace
    assert_set_to_null(
        Some(original.clone()),
        "object1.array2.0",
        json!({
            "object1": {
                "object2": {},
                "array2": [null],
            },
            "array1": original_array1,
        }),
    );
    // Check root.array1[1] replace
    assert_set_to_null(
        Some(original.clone()),
        "array1.1",
        json!({
            "object1": original_object1,
            "array1": [
                {},
                null,
            ],
        }),
    );
    // Check root.array1[] append
    assert_set_to_null(
        Some(original.clone()),
        "array1.",
        json!({
            "object1": original_object1,
            "array1": [
                {},
                [],
                null,
            ],
        }),
    );
    // Check root.array1[0] insert
    assert_set_to_null(
        Some(original.clone()),
        "array1.0.new",
        json!({
            "object1": original_object1,
            "array1": [
                { "new": null },
                [],
            ],
        }),
    );
    // Check root.array1[1][0] insert
    assert_set_to_null(
        Some(original.clone()),
        "array1.1.0",
        json!({
            "object1": original_object1,
            "array1": [
                {},
                [null],
            ],
        }),
    );
    // Check specials
    assert_set_to_null(Some(json!({})), "", json!(null));
    assert_set_to_null(Some(json!({})), ".", json!({ "": null }));
    assert_set_to_null(Some(json!({})), "a", json!({ "a": null }));
    assert_set_to_null(Some(json!([])), "", json!(null));
    assert_set_to_null(Some(json!([])), ".", json!([null]));
    assert_set_to_null(Some(json!([])), "0", json!([null]));
    assert_set_to_null(None, "", json!(null));
    assert_set_to_null(None, ".", json!([null])); // weird, but valid ?
    assert_set_to_null(None, "a", json!({ "a": null }));
    assert_set_to_null(None, "0", json!([null]));
    assert_set_to_null(None, "a..b.0", json!({ "a": [{ "b": [null] }] }));
}

fn assert_set_to_null(node: Option<Value>, path: &str, expected: Value) {
    assert_eq!(
        ToolboxIdlPath::try_parse(path)
            .unwrap()
            .try_set_json_value(node, json!(null))
            .unwrap(),
        expected
    );
}
