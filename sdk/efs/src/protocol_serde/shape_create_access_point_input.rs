// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_access_point_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_access_point::CreateAccessPointInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.file_system_id {
        object.key("FileSystemId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.posix_user {
        #[allow(unused_mut)]
        let mut object_4 = object.key("PosixUser").start_object();
        crate::protocol_serde::shape_posix_user::ser_posix_user(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.root_directory {
        #[allow(unused_mut)]
        let mut object_6 = object.key("RootDirectory").start_object();
        crate::protocol_serde::shape_root_directory::ser_root_directory(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.tags {
        let mut array_8 = object.key("Tags").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    Ok(())
}
