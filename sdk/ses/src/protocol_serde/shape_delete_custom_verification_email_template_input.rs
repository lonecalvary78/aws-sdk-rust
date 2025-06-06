// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_custom_verification_email_template_input_input_input(
    input: &crate::operation::delete_custom_verification_email_template::DeleteCustomVerificationEmailTemplateInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DeleteCustomVerificationEmailTemplate", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TemplateName");
    if let Some(var_2) = &input.template_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
