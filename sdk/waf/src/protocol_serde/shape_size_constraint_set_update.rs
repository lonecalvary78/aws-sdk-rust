// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_size_constraint_set_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SizeConstraintSetUpdate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Action").string(input.action.as_str());
    }
    if let Some(var_1) = &input.size_constraint {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SizeConstraint").start_object();
        crate::protocol_serde::shape_size_constraint::ser_size_constraint(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
