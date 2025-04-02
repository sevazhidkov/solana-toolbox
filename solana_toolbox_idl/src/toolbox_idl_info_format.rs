// TODO (FAR) - support exporting for typescript name convention ?
#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlInfoFormat {
    Human,
    Anchor26,
    Anchor30,
}

impl ToolboxIdlInfoFormat {
    pub fn use_object_for_unordered_named_array(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => false,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn use_root_as_metadata_object(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => false,
            ToolboxIdlInfoFormat::Anchor26 => true,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn use_camel_case_account_flags(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => false,
            ToolboxIdlInfoFormat::Anchor26 => true,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn use_camel_case_primitive_names(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => false,
            ToolboxIdlInfoFormat::Anchor26 => true,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn can_skip_defined_name_object_wrapping(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => true,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn can_skip_type_object_wrapping(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => false,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn can_skip_kind_key(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => false,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn can_shortcut_vec_and_array_notation(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => false,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn can_shortcut_enum_variant_to_string_if_no_field(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => false,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn can_shortcut_defined_name_to_string_if_no_generic(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => false,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }

    pub fn can_shortcut_error_to_number_if_no_msg(&self) -> bool {
        match self {
            ToolboxIdlInfoFormat::Human => true,
            ToolboxIdlInfoFormat::Anchor26 => false,
            ToolboxIdlInfoFormat::Anchor30 => false,
        }
    }
}
