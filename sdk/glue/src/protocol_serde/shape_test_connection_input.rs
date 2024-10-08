// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_test_connection_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::test_connection::TestConnectionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.connection_name {
        object.key("ConnectionName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.test_connection_input {
        #[allow(unused_mut)]
        let mut object_3 = object.key("TestConnectionInput").start_object();
        crate::protocol_serde::shape_test_connection_input::ser_test_connection_input(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

pub fn ser_test_connection_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TestConnectionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("ConnectionType").string(input.connection_type.as_str());
    }
    {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ConnectionProperties").start_object();
        for (key_5, value_6) in &input.connection_properties {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.authentication_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("AuthenticationConfiguration").start_object();
        crate::protocol_serde::shape_authentication_configuration_input::ser_authentication_configuration_input(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
