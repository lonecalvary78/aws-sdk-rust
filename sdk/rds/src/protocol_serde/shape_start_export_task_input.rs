// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_export_task_input_input_input(
    input: &crate::operation::start_export_task::StartExportTaskInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "StartExportTask", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ExportTaskIdentifier");
    if let Some(var_2) = &input.export_task_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SourceArn");
    if let Some(var_4) = &input.source_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("S3BucketName");
    if let Some(var_6) = &input.s3_bucket_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("IamRoleArn");
    if let Some(var_8) = &input.iam_role_arn {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("KmsKeyId");
    if let Some(var_10) = &input.kms_key_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("S3Prefix");
    if let Some(var_12) = &input.s3_prefix {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("ExportOnly");
    if let Some(var_14) = &input.export_only {
        let mut list_16 = scope_13.start_list(false, None);
        for item_15 in var_14 {
            #[allow(unused_mut)]
            let mut entry_17 = list_16.entry();
            entry_17.string(item_15);
        }
        list_16.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
