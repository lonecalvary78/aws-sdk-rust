// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_global_replication_group_input_input_input(
    input: &crate::operation::create_global_replication_group::CreateGlobalReplicationGroupInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateGlobalReplicationGroup", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("GlobalReplicationGroupIdSuffix");
    if let Some(var_2) = &input.global_replication_group_id_suffix {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("GlobalReplicationGroupDescription");
    if let Some(var_4) = &input.global_replication_group_description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PrimaryReplicationGroupId");
    if let Some(var_6) = &input.primary_replication_group_id {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
