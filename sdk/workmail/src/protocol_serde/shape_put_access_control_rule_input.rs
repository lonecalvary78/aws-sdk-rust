// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_access_control_rule_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_access_control_rule::PutAccessControlRuleInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.effect {
        object.key("Effect").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.ip_ranges {
        let mut array_5 = object.key("IpRanges").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.not_ip_ranges {
        let mut array_8 = object.key("NotIpRanges").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.actions {
        let mut array_11 = object.key("Actions").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.not_actions {
        let mut array_14 = object.key("NotActions").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.user_ids {
        let mut array_17 = object.key("UserIds").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.not_user_ids {
        let mut array_20 = object.key("NotUserIds").start_array();
        for item_21 in var_19 {
            {
                array_20.value().string(item_21.as_str());
            }
        }
        array_20.finish();
    }
    if let Some(var_22) = &input.organization_id {
        object.key("OrganizationId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.impersonation_role_ids {
        let mut array_24 = object.key("ImpersonationRoleIds").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.not_impersonation_role_ids {
        let mut array_27 = object.key("NotImpersonationRoleIds").start_array();
        for item_28 in var_26 {
            {
                array_27.value().string(item_28.as_str());
            }
        }
        array_27.finish();
    }
    Ok(())
}
