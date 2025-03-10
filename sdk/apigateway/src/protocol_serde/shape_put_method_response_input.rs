// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_method_response_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_method_response::PutMethodResponseInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.response_models {
        #[allow(unused_mut)]
        let mut object_2 = object.key("responseModels").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.response_parameters {
        #[allow(unused_mut)]
        let mut object_6 = object.key("responseParameters").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).boolean(*value_8);
            }
        }
        object_6.finish();
    }
    Ok(())
}
