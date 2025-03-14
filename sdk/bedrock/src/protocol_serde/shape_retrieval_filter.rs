// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_retrieval_filter(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RetrievalFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::RetrievalFilter::Equals(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("equals").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::RetrievalFilter::NotEquals(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_4.key("notEquals").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::RetrievalFilter::GreaterThan(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_4.key("greaterThan").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::RetrievalFilter::GreaterThanOrEquals(inner) => {
            #[allow(unused_mut)]
            let mut object_4 = object_4.key("greaterThanOrEquals").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_4, inner)?;
            object_4.finish();
        }
        crate::types::RetrievalFilter::LessThan(inner) => {
            #[allow(unused_mut)]
            let mut object_5 = object_4.key("lessThan").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_5, inner)?;
            object_5.finish();
        }
        crate::types::RetrievalFilter::LessThanOrEquals(inner) => {
            #[allow(unused_mut)]
            let mut object_6 = object_4.key("lessThanOrEquals").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_6, inner)?;
            object_6.finish();
        }
        crate::types::RetrievalFilter::In(inner) => {
            #[allow(unused_mut)]
            let mut object_7 = object_4.key("in").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_7, inner)?;
            object_7.finish();
        }
        crate::types::RetrievalFilter::NotIn(inner) => {
            #[allow(unused_mut)]
            let mut object_8 = object_4.key("notIn").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_8, inner)?;
            object_8.finish();
        }
        crate::types::RetrievalFilter::StartsWith(inner) => {
            #[allow(unused_mut)]
            let mut object_9 = object_4.key("startsWith").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_9, inner)?;
            object_9.finish();
        }
        crate::types::RetrievalFilter::ListContains(inner) => {
            #[allow(unused_mut)]
            let mut object_10 = object_4.key("listContains").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_10, inner)?;
            object_10.finish();
        }
        crate::types::RetrievalFilter::StringContains(inner) => {
            #[allow(unused_mut)]
            let mut object_11 = object_4.key("stringContains").start_object();
            crate::protocol_serde::shape_filter_attribute::ser_filter_attribute(&mut object_11, inner)?;
            object_11.finish();
        }
        crate::types::RetrievalFilter::AndAll(inner) => {
            let mut array_12 = object_4.key("andAll").start_array();
            for item_13 in inner {
                {
                    #[allow(unused_mut)]
                    let mut object_14 = array_12.value().start_object();
                    crate::protocol_serde::shape_retrieval_filter::ser_retrieval_filter(&mut object_14, item_13)?;
                    object_14.finish();
                }
            }
            array_12.finish();
        }
        crate::types::RetrievalFilter::OrAll(inner) => {
            let mut array_15 = object_4.key("orAll").start_array();
            for item_16 in inner {
                {
                    #[allow(unused_mut)]
                    let mut object_17 = array_15.value().start_object();
                    crate::protocol_serde::shape_retrieval_filter::ser_retrieval_filter(&mut object_17, item_16)?;
                    object_17.finish();
                }
            }
            array_15.finish();
        }
        crate::types::RetrievalFilter::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "RetrievalFilter",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_retrieval_filter<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RetrievalFilter>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
                        tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                        "equals" => Some(crate::types::RetrievalFilter::Equals(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'equals' cannot be null")
                            })?,
                        )),
                        "notEquals" => Some(crate::types::RetrievalFilter::NotEquals(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'notEquals' cannot be null")
                            })?,
                        )),
                        "greaterThan" => Some(crate::types::RetrievalFilter::GreaterThan(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'greaterThan' cannot be null")
                            })?,
                        )),
                        "greaterThanOrEquals" => Some(crate::types::RetrievalFilter::GreaterThanOrEquals(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'greaterThanOrEquals' cannot be null")
                            })?,
                        )),
                        "lessThan" => Some(crate::types::RetrievalFilter::LessThan(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'lessThan' cannot be null")
                            })?,
                        )),
                        "lessThanOrEquals" => Some(crate::types::RetrievalFilter::LessThanOrEquals(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'lessThanOrEquals' cannot be null")
                            })?,
                        )),
                        "in" => Some(crate::types::RetrievalFilter::In(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'in' cannot be null"))?,
                        )),
                        "notIn" => Some(crate::types::RetrievalFilter::NotIn(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'notIn' cannot be null"))?,
                        )),
                        "startsWith" => Some(crate::types::RetrievalFilter::StartsWith(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'startsWith' cannot be null")
                            })?,
                        )),
                        "listContains" => Some(crate::types::RetrievalFilter::ListContains(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'listContains' cannot be null")
                            })?,
                        )),
                        "stringContains" => Some(crate::types::RetrievalFilter::StringContains(
                            crate::protocol_serde::shape_filter_attribute::de_filter_attribute(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'stringContains' cannot be null")
                            })?,
                        )),
                        "andAll" => Some(crate::types::RetrievalFilter::AndAll(
                            crate::protocol_serde::shape_retrieval_filter_list::de_retrieval_filter_list(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'andAll' cannot be null")
                            })?,
                        )),
                        "orAll" => Some(crate::types::RetrievalFilter::OrAll(
                            crate::protocol_serde::shape_retrieval_filter_list::de_retrieval_filter_list(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'orAll' cannot be null"))?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::RetrievalFilter::Unknown)
                        }
                    };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
