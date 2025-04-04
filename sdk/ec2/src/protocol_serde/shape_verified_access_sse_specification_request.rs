// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_verified_access_sse_specification_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::VerifiedAccessSseSpecificationRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CustomerManagedKeyEnabled");
    if let Some(var_2) = &input.customer_managed_key_enabled {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("KmsKeyArn");
    if let Some(var_4) = &input.kms_key_arn {
        scope_3.string(var_4);
    }
    Ok(())
}
