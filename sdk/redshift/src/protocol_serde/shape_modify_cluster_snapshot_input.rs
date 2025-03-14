// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_cluster_snapshot_input_input_input(
    input: &crate::operation::modify_cluster_snapshot::ModifyClusterSnapshotInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyClusterSnapshot", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SnapshotIdentifier");
    if let Some(var_2) = &input.snapshot_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ManualSnapshotRetentionPeriod");
    if let Some(var_4) = &input.manual_snapshot_retention_period {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Force");
    if let Some(var_6) = &input.force {
        scope_5.boolean(*var_6);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
