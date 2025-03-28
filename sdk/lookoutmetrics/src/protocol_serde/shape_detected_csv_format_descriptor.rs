// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_detected_csv_format_descriptor<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::DetectedCsvFormatDescriptor>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::DetectedCsvFormatDescriptorBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FileCompression" => {
                            builder = builder.set_file_compression(crate::protocol_serde::shape_detected_field::de_detected_field(tokens)?);
                        }
                        "Charset" => {
                            builder = builder.set_charset(crate::protocol_serde::shape_detected_field::de_detected_field(tokens)?);
                        }
                        "ContainsHeader" => {
                            builder = builder.set_contains_header(crate::protocol_serde::shape_detected_field::de_detected_field(tokens)?);
                        }
                        "Delimiter" => {
                            builder = builder.set_delimiter(crate::protocol_serde::shape_detected_field::de_detected_field(tokens)?);
                        }
                        "HeaderList" => {
                            builder = builder.set_header_list(crate::protocol_serde::shape_detected_field::de_detected_field(tokens)?);
                        }
                        "QuoteSymbol" => {
                            builder = builder.set_quote_symbol(crate::protocol_serde::shape_detected_field::de_detected_field(tokens)?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
