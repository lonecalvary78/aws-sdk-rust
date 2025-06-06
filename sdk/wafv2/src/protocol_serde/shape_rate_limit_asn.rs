// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rate_limit_asn(
    #[allow(unused_variables)] object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    #[allow(unused_variables)] input: &crate::types::RateLimitAsn,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    Ok(())
}

pub(crate) fn de_rate_limit_asn<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RateLimitAsn>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RateLimitAsnBuilder::default();
            ::aws_smithy_json::deserialize::token::skip_to_end(tokens)?;
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
