use solana_sdk::bpf_loader_upgradeable;
use solana_sdk::compute_budget;
use solana_sdk::system_program;
use solana_toolbox_idl::ToolboxIdlInfoFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Fetch standard IDL for the native system program
    let idl_program_native_system =
        ToolboxIdlProgram::from_lib(&system_program::ID).unwrap();
    // Test that it's equivalent to the original IDL after being exported
    assert_eq!(
        idl_program_native_system,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_system.export(&ToolboxIdlInfoFormat::Human)
        )
        .unwrap()
    );
    assert_eq!(
        idl_program_native_system,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_system.export(&ToolboxIdlInfoFormat::Anchor26)
        )
        .unwrap()
    );
    assert_eq!(
        idl_program_native_system,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_system.export(&ToolboxIdlInfoFormat::Anchor30)
        )
        .unwrap()
    );
    // Fetch standard IDL for the native compute_budget program
    let idl_program_native_compute_budget =
        ToolboxIdlProgram::from_lib(&compute_budget::ID).unwrap();
    // Test that it's equivalent to the original IDL after being exported
    assert_eq!(
        idl_program_native_compute_budget,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_compute_budget
                .export(&ToolboxIdlInfoFormat::Human)
        )
        .unwrap()
    );
    assert_eq!(
        idl_program_native_compute_budget,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_compute_budget
                .export(&ToolboxIdlInfoFormat::Anchor26)
        )
        .unwrap()
    );
    assert_eq!(
        idl_program_native_compute_budget,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_compute_budget
                .export(&ToolboxIdlInfoFormat::Anchor30)
        )
        .unwrap()
    );
    // Fetch standard IDL for the native loader_upgradeable program
    let idl_program_native_loader_upgradeable =
        ToolboxIdlProgram::from_lib(&bpf_loader_upgradeable::ID).unwrap();
    // Test that it's equivalent to the original IDL after being exported
    assert_eq!(
        idl_program_native_loader_upgradeable,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_loader_upgradeable
                .export(&ToolboxIdlInfoFormat::Human)
        )
        .unwrap()
    );
    assert_eq!(
        idl_program_native_loader_upgradeable,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_loader_upgradeable
                .export(&ToolboxIdlInfoFormat::Anchor26)
        )
        .unwrap()
    );
    assert_eq!(
        idl_program_native_loader_upgradeable,
        ToolboxIdlProgram::try_parse_from_value(
            &idl_program_native_loader_upgradeable
                .export(&ToolboxIdlInfoFormat::Anchor30)
        )
        .unwrap()
    );
}
