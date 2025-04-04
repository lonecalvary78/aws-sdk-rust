// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_user_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_user::CreateUserInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.email_address {
        object.key("EmailAddress").string(var_1.as_str());
    }
    if let Some(var_2) = &input.given_name {
        object.key("GivenName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.organization_id {
        object.key("OrganizationId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.password {
        object.key("Password").string(var_4.as_str());
    }
    if let Some(var_5) = &input.storage_rule {
        #[allow(unused_mut)]
        let mut object_6 = object.key("StorageRule").start_object();
        crate::protocol_serde::shape_storage_rule_type::ser_storage_rule_type(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.surname {
        object.key("Surname").string(var_7.as_str());
    }
    if let Some(var_8) = &input.time_zone_id {
        object.key("TimeZoneId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.username {
        object.key("Username").string(var_9.as_str());
    }
    Ok(())
}
