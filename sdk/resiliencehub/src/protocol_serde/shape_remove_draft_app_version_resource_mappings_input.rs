// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_draft_app_version_resource_mappings_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::remove_draft_app_version_resource_mappings::RemoveDraftAppVersionResourceMappingsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.app_arn {
        object.key("appArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.app_registry_app_names {
        let mut array_3 = object.key("appRegistryAppNames").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.eks_source_names {
        let mut array_6 = object.key("eksSourceNames").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.logical_stack_names {
        let mut array_9 = object.key("logicalStackNames").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.resource_group_names {
        let mut array_12 = object.key("resourceGroupNames").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.resource_names {
        let mut array_15 = object.key("resourceNames").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.terraform_source_names {
        let mut array_18 = object.key("terraformSourceNames").start_array();
        for item_19 in var_17 {
            {
                array_18.value().string(item_19.as_str());
            }
        }
        array_18.finish();
    }
    Ok(())
}
