// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_file_cache_nfs_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FileCacheNfsConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.version {
        object.key("Version").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dns_ips {
        let mut array_3 = object.key("DnsIps").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}
