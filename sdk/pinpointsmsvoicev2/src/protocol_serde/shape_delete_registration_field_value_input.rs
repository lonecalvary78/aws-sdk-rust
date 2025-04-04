// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_registration_field_value_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_registration_field_value::DeleteRegistrationFieldValueInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.registration_id {
        object.key("RegistrationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.field_path {
        object.key("FieldPath").string(var_2.as_str());
    }
    Ok(())
}
