// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_import_image_license_configuration_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ImportImageLicenseConfigurationRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LicenseConfigurationArn");
    if let Some(var_2) = &input.license_configuration_arn {
        scope_1.string(var_2);
    }
    Ok(())
}
