// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lte_network_measurements(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LteNetworkMeasurements,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Earfcn").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.earfcn).into()),
        );
    }
    {
        object.key("CellId").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.cell_id).into()),
        );
    }
    {
        object.key("Pci").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.pci).into()),
        );
    }
    if let Some(var_1) = &input.rsrp {
        object.key("Rsrp").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.rsrq {
        object.key("Rsrq").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_2).into()),
        );
    }
    Ok(())
}
