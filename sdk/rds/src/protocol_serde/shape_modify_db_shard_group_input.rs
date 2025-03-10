// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_db_shard_group_input_input_input(
    input: &crate::operation::modify_db_shard_group::ModifyDbShardGroupInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyDBShardGroup", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBShardGroupIdentifier");
    if let Some(var_2) = &input.db_shard_group_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MaxACU");
    if let Some(var_4) = &input.max_acu {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MinACU");
    if let Some(var_6) = &input.min_acu {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("ComputeRedundancy");
    if let Some(var_8) = &input.compute_redundancy {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
