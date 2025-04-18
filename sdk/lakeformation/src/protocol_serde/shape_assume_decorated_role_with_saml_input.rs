// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_assume_decorated_role_with_saml_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::assume_decorated_role_with_saml::AssumeDecoratedRoleWithSamlInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.duration_seconds {
        object.key("DurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.principal_arn {
        object.key("PrincipalArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.role_arn {
        object.key("RoleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.saml_assertion {
        object.key("SAMLAssertion").string(var_4.as_str());
    }
    Ok(())
}
