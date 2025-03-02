// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_delivery_channel_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_delivery_channel::PutDeliveryChannelInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.delivery_channel {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DeliveryChannel").start_object();
        crate::protocol_serde::shape_delivery_channel::ser_delivery_channel(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
