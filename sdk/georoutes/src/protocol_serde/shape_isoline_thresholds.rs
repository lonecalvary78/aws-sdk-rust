// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_isoline_thresholds(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IsolineThresholds,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.distance {
        let mut array_2 = object.key("Distance").start_array();
        for item_3 in var_1 {
            {
                array_2.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::NegInt((*item_3).into()),
                );
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.time {
        let mut array_5 = object.key("Time").start_array();
        for item_6 in var_4 {
            {
                array_5.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::NegInt((*item_6).into()),
                );
            }
        }
        array_5.finish();
    }
    Ok(())
}
