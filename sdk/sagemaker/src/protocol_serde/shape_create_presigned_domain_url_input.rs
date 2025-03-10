// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_presigned_domain_url_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_presigned_domain_url::CreatePresignedDomainUrlInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.domain_id {
        object.key("DomainId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.user_profile_name {
        object.key("UserProfileName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.session_expiration_duration_in_seconds {
        object.key("SessionExpirationDurationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.expires_in_seconds {
        object.key("ExpiresInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.space_name {
        object.key("SpaceName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.landing_uri {
        object.key("LandingUri").string(var_6.as_str());
    }
    Ok(())
}
