// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_copy_db_parameter_group_input_input_input(
    input: &crate::operation::copy_db_parameter_group::CopyDbParameterGroupInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CopyDBParameterGroup", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SourceDBParameterGroupIdentifier");
    if let Some(var_2) = &input.source_db_parameter_group_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TargetDBParameterGroupIdentifier");
    if let Some(var_4) = &input.target_db_parameter_group_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TargetDBParameterGroupDescription");
    if let Some(var_6) = &input.target_db_parameter_group_description {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Tags");
    if let Some(var_8) = &input.tags {
        let mut list_10 = scope_7.start_list(false, Some("Tag"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_11, item_9)?;
        }
        list_10.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
