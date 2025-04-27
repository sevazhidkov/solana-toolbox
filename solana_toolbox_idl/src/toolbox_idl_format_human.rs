use crate::toolbox_idl_format::ToolboxIdlFormat;

impl ToolboxIdlFormat {
    pub fn human() -> ToolboxIdlFormat {
        ToolboxIdlFormat {
            use_object_for_unordered_named_array: true,
            use_root_also_as_metadata_object: false,
            use_camel_case_instruction_names: false,
            use_camel_case_instruction_account_names: false,
            use_camel_case_instruction_account_flags: false,
            use_camel_case_type_primitive_names: false,
            use_camel_case_type_fields_names: false,
            can_skip_defined_name_object_wrap: true,
            can_skip_unnamed_field_type_object_wrap: true,
            can_skip_typedef_type_object_wrap: true,
            can_skip_typedef_generic_kind_key: true,
            can_skip_type_kind_key: true,
            can_skip_instruction_account_pda_kind_key: true,
            can_skip_instruction_account_pda_type_key: true,
            can_shortcut_type_vec_notation: true,
            can_shortcut_type_array_notation: true,
            can_shortcut_enum_variant_to_string_if_no_fields: true,
            can_shortcut_defined_name_to_string_if_no_generic: true,
            can_shortcut_error_to_number_if_no_msg: true,
        }
    }
}
