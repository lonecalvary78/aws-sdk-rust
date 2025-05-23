// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_tenant_database_pending_modified_values(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::TenantDatabasePendingModifiedValues, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TenantDatabasePendingModifiedValues::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("MasterUserPassword") /* MasterUserPassword com.amazonaws.rds#TenantDatabasePendingModifiedValues$MasterUserPassword */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_master_user_password(var_1);
            }
            ,
            s if s.matches("TenantDBName") /* TenantDBName com.amazonaws.rds#TenantDatabasePendingModifiedValues$TenantDBName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_tenant_db_name(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
