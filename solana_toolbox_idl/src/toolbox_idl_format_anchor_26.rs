use crate::toolbox_idl_format::ToolboxIdlFormat;

impl ToolboxIdlFormat {
    pub fn anchor_26() -> ToolboxIdlFormat {
        ToolboxIdlFormat {
            use_object_for_unordered_named_array: false,
            use_root_as_metadata_object: true,
            use_camel_case_instruction_names: true,
            use_camel_case_instruction_account_names: true,
            use_camel_case_instruction_account_flags: true,
            use_camel_case_type_primitive_names: true,
            use_camel_case_type_fields_names: true,
            can_skip_defined_name_object_wrap: true,
            can_skip_unamed_field_type_object_wrap: false,
            can_skip_typedef_type_object_wrap: false,
            can_skip_generic_kind_key: false,
            can_skip_type_kind_key: false,
            can_skip_instruction_account_pda_kind_key: false,
            can_skip_instruction_account_pda_type_key: false,
            can_shortcut_vec_notation: false,
            can_shortcut_array_notation: false,
            can_shortcut_enum_variant_to_string_if_no_fields: false,
            can_shortcut_defined_name_to_string_if_no_generic: false,
            can_shortcut_error_to_number_if_no_msg: false,
        }
    }
}
