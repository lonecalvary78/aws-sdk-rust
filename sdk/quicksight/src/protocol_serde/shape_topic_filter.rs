// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_topic_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TopicFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.filter_description {
        object.key("FilterDescription").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter_class {
        object.key("FilterClass").string(var_2.as_str());
    }
    {
        object.key("FilterName").string(input.filter_name.as_str());
    }
    if let Some(var_3) = &input.filter_synonyms {
        let mut array_4 = object.key("FilterSynonyms").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    {
        object.key("OperandFieldName").string(input.operand_field_name.as_str());
    }
    if let Some(var_6) = &input.filter_type {
        object.key("FilterType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.category_filter {
        #[allow(unused_mut)]
        let mut object_8 = object.key("CategoryFilter").start_object();
        crate::protocol_serde::shape_topic_category_filter::ser_topic_category_filter(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.numeric_equality_filter {
        #[allow(unused_mut)]
        let mut object_10 = object.key("NumericEqualityFilter").start_object();
        crate::protocol_serde::shape_topic_numeric_equality_filter::ser_topic_numeric_equality_filter(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.numeric_range_filter {
        #[allow(unused_mut)]
        let mut object_12 = object.key("NumericRangeFilter").start_object();
        crate::protocol_serde::shape_topic_numeric_range_filter::ser_topic_numeric_range_filter(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.date_range_filter {
        #[allow(unused_mut)]
        let mut object_14 = object.key("DateRangeFilter").start_object();
        crate::protocol_serde::shape_topic_date_range_filter::ser_topic_date_range_filter(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.relative_date_filter {
        #[allow(unused_mut)]
        let mut object_16 = object.key("RelativeDateFilter").start_object();
        crate::protocol_serde::shape_topic_relative_date_filter::ser_topic_relative_date_filter(&mut object_16, var_15)?;
        object_16.finish();
    }
    Ok(())
}

pub(crate) fn de_topic_filter<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TopicFilter>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TopicFilterBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FilterDescription" => {
                            builder = builder.set_filter_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FilterClass" => {
                            builder = builder.set_filter_class(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::FilterClass::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "FilterName" => {
                            builder = builder.set_filter_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FilterSynonyms" => {
                            builder = builder.set_filter_synonyms(crate::protocol_serde::shape_synonyms::de_synonyms(tokens)?);
                        }
                        "OperandFieldName" => {
                            builder = builder.set_operand_field_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FilterType" => {
                            builder = builder.set_filter_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::NamedFilterType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "CategoryFilter" => {
                            builder =
                                builder.set_category_filter(crate::protocol_serde::shape_topic_category_filter::de_topic_category_filter(tokens)?);
                        }
                        "NumericEqualityFilter" => {
                            builder = builder.set_numeric_equality_filter(
                                crate::protocol_serde::shape_topic_numeric_equality_filter::de_topic_numeric_equality_filter(tokens)?,
                            );
                        }
                        "NumericRangeFilter" => {
                            builder = builder.set_numeric_range_filter(
                                crate::protocol_serde::shape_topic_numeric_range_filter::de_topic_numeric_range_filter(tokens)?,
                            );
                        }
                        "DateRangeFilter" => {
                            builder = builder
                                .set_date_range_filter(crate::protocol_serde::shape_topic_date_range_filter::de_topic_date_range_filter(tokens)?);
                        }
                        "RelativeDateFilter" => {
                            builder = builder.set_relative_date_filter(
                                crate::protocol_serde::shape_topic_relative_date_filter::de_topic_relative_date_filter(tokens)?,
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
            Ok(Some(crate::serde_util::topic_filter_correct_errors(builder).build().map_err(|err| {
                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
            })?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
