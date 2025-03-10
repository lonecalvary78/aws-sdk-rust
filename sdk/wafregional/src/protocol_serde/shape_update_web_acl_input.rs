// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_web_acl_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_web_acl::UpdateWebAclInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.web_acl_id {
        object.key("WebACLId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.change_token {
        object.key("ChangeToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.updates {
        let mut array_4 = object.key("Updates").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_web_acl_update::ser_web_acl_update(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.default_action {
        #[allow(unused_mut)]
        let mut object_8 = object.key("DefaultAction").start_object();
        crate::protocol_serde::shape_waf_action::ser_waf_action(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
