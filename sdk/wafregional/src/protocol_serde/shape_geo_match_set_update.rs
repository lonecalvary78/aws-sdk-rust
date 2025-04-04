// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_geo_match_set_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GeoMatchSetUpdate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Action").string(input.action.as_str());
    }
    if let Some(var_1) = &input.geo_match_constraint {
        #[allow(unused_mut)]
        let mut object_2 = object.key("GeoMatchConstraint").start_object();
        crate::protocol_serde::shape_geo_match_constraint::ser_geo_match_constraint(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
