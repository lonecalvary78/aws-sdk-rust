// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_policy_definition(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PolicyDefinition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::PolicyDefinition::Static(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("static").start_object();
            crate::protocol_serde::shape_static_policy_definition::ser_static_policy_definition(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::PolicyDefinition::TemplateLinked(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_4.key("templateLinked").start_object();
            crate::protocol_serde::shape_template_linked_policy_definition::ser_template_linked_policy_definition(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::PolicyDefinition::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "PolicyDefinition",
            ))
        }
    }
    Ok(())
}
