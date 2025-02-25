// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_channel_namespace_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_channel_namespace::CreateChannelNamespaceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.code_handlers {
        object.key("codeHandlers").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.publish_auth_modes {
        let mut array_4 = object.key("publishAuthModes").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_auth_mode::ser_auth_mode(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.subscribe_auth_modes {
        let mut array_8 = object.key("subscribeAuthModes").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_auth_mode::ser_auth_mode(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.tags {
        #[allow(unused_mut)]
        let mut object_12 = object.key("tags").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    Ok(())
}
