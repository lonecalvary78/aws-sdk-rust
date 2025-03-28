// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_cluster_pending_modified_values(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ClusterPendingModifiedValues, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ClusterPendingModifiedValues::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("PendingCloudwatchLogsExports") /* PendingCloudwatchLogsExports com.amazonaws.rds#ClusterPendingModifiedValues$PendingCloudwatchLogsExports */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_pending_cloudwatch_logs_exports::de_pending_cloudwatch_logs_exports(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_pending_cloudwatch_logs_exports(var_1);
            }
            ,
            s if s.matches("DBClusterIdentifier") /* DBClusterIdentifier com.amazonaws.rds#ClusterPendingModifiedValues$DBClusterIdentifier */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_identifier(var_2);
            }
            ,
            s if s.matches("MasterUserPassword") /* MasterUserPassword com.amazonaws.rds#ClusterPendingModifiedValues$MasterUserPassword */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_master_user_password(var_3);
            }
            ,
            s if s.matches("IAMDatabaseAuthenticationEnabled") /* IAMDatabaseAuthenticationEnabled com.amazonaws.rds#ClusterPendingModifiedValues$IAMDatabaseAuthenticationEnabled */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_iam_database_authentication_enabled(var_4);
            }
            ,
            s if s.matches("EngineVersion") /* EngineVersion com.amazonaws.rds#ClusterPendingModifiedValues$EngineVersion */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine_version(var_5);
            }
            ,
            s if s.matches("BackupRetentionPeriod") /* BackupRetentionPeriod com.amazonaws.rds#ClusterPendingModifiedValues$BackupRetentionPeriod */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.rds#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_backup_retention_period(var_6);
            }
            ,
            s if s.matches("AllocatedStorage") /* AllocatedStorage com.amazonaws.rds#ClusterPendingModifiedValues$AllocatedStorage */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.rds#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_allocated_storage(var_7);
            }
            ,
            s if s.matches("RdsCustomClusterConfiguration") /* RdsCustomClusterConfiguration com.amazonaws.rds#ClusterPendingModifiedValues$RdsCustomClusterConfiguration */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_rds_custom_cluster_configuration::de_rds_custom_cluster_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_rds_custom_cluster_configuration(var_8);
            }
            ,
            s if s.matches("Iops") /* Iops com.amazonaws.rds#ClusterPendingModifiedValues$Iops */ =>  {
                let var_9 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.rds#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_iops(var_9);
            }
            ,
            s if s.matches("StorageType") /* StorageType com.amazonaws.rds#ClusterPendingModifiedValues$StorageType */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_type(var_10);
            }
            ,
            s if s.matches("CertificateDetails") /* CertificateDetails com.amazonaws.rds#ClusterPendingModifiedValues$CertificateDetails */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_certificate_details::de_certificate_details(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_certificate_details(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
