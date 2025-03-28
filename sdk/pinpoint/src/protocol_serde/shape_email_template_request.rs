// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_email_template_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EmailTemplateRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.default_substitutions {
        object.key("DefaultSubstitutions").string(var_1.as_str());
    }
    if let Some(var_2) = &input.html_part {
        object.key("HtmlPart").string(var_2.as_str());
    }
    if let Some(var_3) = &input.recommender_id {
        object.key("RecommenderId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.subject {
        object.key("Subject").string(var_4.as_str());
    }
    if let Some(var_5) = &input.headers {
        let mut array_6 = object.key("Headers").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_message_header::ser_message_header(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.template_description {
        object.key("TemplateDescription").string(var_13.as_str());
    }
    if let Some(var_14) = &input.text_part {
        object.key("TextPart").string(var_14.as_str());
    }
    Ok(())
}
