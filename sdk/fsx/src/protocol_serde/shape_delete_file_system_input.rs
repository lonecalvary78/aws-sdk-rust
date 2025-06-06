// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_file_system_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_file_system::DeleteFileSystemInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.file_system_id {
        object.key("FileSystemId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.windows_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("WindowsConfiguration").start_object();
        crate::protocol_serde::shape_delete_file_system_windows_configuration::ser_delete_file_system_windows_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.lustre_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("LustreConfiguration").start_object();
        crate::protocol_serde::shape_delete_file_system_lustre_configuration::ser_delete_file_system_lustre_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.open_zfs_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("OpenZFSConfiguration").start_object();
        crate::protocol_serde::shape_delete_file_system_open_zfs_configuration::ser_delete_file_system_open_zfs_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
