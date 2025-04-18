// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_bgp_failover_test_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_bgp_failover_test::StartBgpFailoverTestInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.virtual_interface_id {
        object.key("virtualInterfaceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.bgp_peers {
        let mut array_3 = object.key("bgpPeers").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.test_duration_in_minutes {
        object.key("testDurationInMinutes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    Ok(())
}
