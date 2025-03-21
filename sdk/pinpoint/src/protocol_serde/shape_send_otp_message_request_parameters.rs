// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_otp_message_request_parameters(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SendOtpMessageRequestParameters,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.allowed_attempts {
        object.key("AllowedAttempts").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.brand_name {
        object.key("BrandName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.channel {
        object.key("Channel").string(var_3.as_str());
    }
    if let Some(var_4) = &input.code_length {
        object.key("CodeLength").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.destination_identity {
        object.key("DestinationIdentity").string(var_5.as_str());
    }
    if let Some(var_6) = &input.entity_id {
        object.key("EntityId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.language {
        object.key("Language").string(var_7.as_str());
    }
    if let Some(var_8) = &input.origination_identity {
        object.key("OriginationIdentity").string(var_8.as_str());
    }
    if let Some(var_9) = &input.reference_id {
        object.key("ReferenceId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.template_id {
        object.key("TemplateId").string(var_10.as_str());
    }
    if let Some(var_11) = &input.validity_period {
        object.key("ValidityPeriod").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    Ok(())
}
