// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_expense_field<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ExpenseField>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ExpenseFieldBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Type" => {
                            builder = builder.set_type(crate::protocol_serde::shape_expense_type::de_expense_type(tokens)?);
                        }
                        "LabelDetection" => {
                            builder = builder.set_label_detection(crate::protocol_serde::shape_expense_detection::de_expense_detection(tokens)?);
                        }
                        "ValueDetection" => {
                            builder = builder.set_value_detection(crate::protocol_serde::shape_expense_detection::de_expense_detection(tokens)?);
                        }
                        "PageNumber" => {
                            builder = builder.set_page_number(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Currency" => {
                            builder = builder.set_currency(crate::protocol_serde::shape_expense_currency::de_expense_currency(tokens)?);
                        }
                        "GroupProperties" => {
                            builder = builder.set_group_properties(
                                crate::protocol_serde::shape_expense_group_property_list::de_expense_group_property_list(tokens)?,
                            );
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
