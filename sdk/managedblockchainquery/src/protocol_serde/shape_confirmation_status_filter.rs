// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_confirmation_status_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ConfirmationStatusFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("include").start_array();
        for item_2 in &input.include {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    Ok(())
}
