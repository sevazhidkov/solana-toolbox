#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlFormat {
    pub use_object_for_unordered_named_array: bool,
    pub use_root_also_as_metadata_object: bool,
    pub use_camel_case_instruction_names: bool,
    pub use_camel_case_instruction_account_names: bool,
    pub use_camel_case_instruction_account_flags: bool,
    pub use_camel_case_type_primitive_names: bool,
    pub use_camel_case_type_fields_names: bool,
    pub can_skip_defined_name_object_wrap: bool,
    pub can_skip_unnamed_field_type_object_wrap: bool,
    pub can_skip_typedef_type_object_wrap: bool,
    pub can_skip_typedef_generic_kind_key: bool,
    pub can_skip_type_kind_key: bool,
    pub can_skip_instruction_account_pda_kind_key: bool,
    pub can_skip_instruction_account_pda_type_key: bool,
    pub can_shortcut_type_vec_notation: bool,
    pub can_shortcut_type_array_notation: bool,
    pub can_shortcut_enum_variant_to_string_if_no_fields: bool,
    pub can_shortcut_defined_name_to_string_if_no_generic: bool,
    pub can_shortcut_error_to_number_if_no_msg: bool, // TODO - should I just deprecate all exporting ?
}
