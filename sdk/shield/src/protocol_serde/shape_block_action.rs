// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_block_action(
    #[allow(unused_variables)] object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    #[allow(unused_variables)] input: &crate::types::BlockAction,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    Ok(())
}

pub(crate) fn de_block_action<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::BlockAction>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::BlockActionBuilder::default();
            ::aws_smithy_json::deserialize::token::skip_to_end(tokens)?;
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
