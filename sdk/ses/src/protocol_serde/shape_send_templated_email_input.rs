// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_templated_email_input_input_input(
    input: &crate::operation::send_templated_email::SendTemplatedEmailInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "SendTemplatedEmail", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Source");
    if let Some(var_2) = &input.source {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Destination");
    if let Some(var_4) = &input.destination {
        crate::protocol_serde::shape_destination::ser_destination(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ReplyToAddresses");
    if let Some(var_6) = &input.reply_to_addresses {
        let mut list_8 = scope_5.start_list(false, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7);
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("ReturnPath");
    if let Some(var_11) = &input.return_path {
        scope_10.string(var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("SourceArn");
    if let Some(var_13) = &input.source_arn {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("ReturnPathArn");
    if let Some(var_15) = &input.return_path_arn {
        scope_14.string(var_15);
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("Tags");
    if let Some(var_17) = &input.tags {
        let mut list_19 = scope_16.start_list(false, None);
        for item_18 in var_17 {
            #[allow(unused_mut)]
            let mut entry_20 = list_19.entry();
            crate::protocol_serde::shape_message_tag::ser_message_tag(entry_20, item_18)?;
        }
        list_19.finish();
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("ConfigurationSetName");
    if let Some(var_22) = &input.configuration_set_name {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("Template");
    if let Some(var_24) = &input.template {
        scope_23.string(var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("TemplateArn");
    if let Some(var_26) = &input.template_arn {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("TemplateData");
    if let Some(var_28) = &input.template_data {
        scope_27.string(var_28);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
