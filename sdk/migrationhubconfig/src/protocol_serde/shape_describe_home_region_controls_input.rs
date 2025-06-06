// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_home_region_controls_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_home_region_controls::DescribeHomeRegionControlsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.control_id {
        object.key("ControlId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.home_region {
        object.key("HomeRegion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Target").start_object();
        crate::protocol_serde::shape_target::ser_target(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    Ok(())
}
