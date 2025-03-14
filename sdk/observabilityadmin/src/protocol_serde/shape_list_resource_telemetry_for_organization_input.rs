// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_resource_telemetry_for_organization_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_resource_telemetry_for_organization::ListResourceTelemetryForOrganizationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.account_identifiers {
        let mut array_2 = object.key("AccountIdentifiers").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.next_token {
        object.key("NextToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.resource_identifier_prefix {
        object.key("ResourceIdentifierPrefix").string(var_6.as_str());
    }
    if let Some(var_7) = &input.resource_tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("ResourceTags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.resource_types {
        let mut array_12 = object.key("ResourceTypes").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.telemetry_configuration_state {
        #[allow(unused_mut)]
        let mut object_15 = object.key("TelemetryConfigurationState").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    Ok(())
}
