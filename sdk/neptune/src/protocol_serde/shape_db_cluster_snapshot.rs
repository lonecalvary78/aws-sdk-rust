// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_db_cluster_snapshot(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::DbClusterSnapshot, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DbClusterSnapshot::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AvailabilityZones") /* AvailabilityZones com.amazonaws.neptune#DBClusterSnapshot$AvailabilityZones */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_availability_zones::de_availability_zones(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_1);
            }
            ,
            s if s.matches("DBClusterSnapshotIdentifier") /* DBClusterSnapshotIdentifier com.amazonaws.neptune#DBClusterSnapshot$DBClusterSnapshotIdentifier */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_snapshot_identifier(var_2);
            }
            ,
            s if s.matches("DBClusterIdentifier") /* DBClusterIdentifier com.amazonaws.neptune#DBClusterSnapshot$DBClusterIdentifier */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_identifier(var_3);
            }
            ,
            s if s.matches("SnapshotCreateTime") /* SnapshotCreateTime com.amazonaws.neptune#DBClusterSnapshot$SnapshotCreateTime */ =>  {
                let var_4 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.neptune#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_snapshot_create_time(var_4);
            }
            ,
            s if s.matches("Engine") /* Engine com.amazonaws.neptune#DBClusterSnapshot$Engine */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine(var_5);
            }
            ,
            s if s.matches("AllocatedStorage") /* AllocatedStorage com.amazonaws.neptune#DBClusterSnapshot$AllocatedStorage */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.neptune#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_allocated_storage(var_6);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.neptune#DBClusterSnapshot$Status */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_7);
            }
            ,
            s if s.matches("Port") /* Port com.amazonaws.neptune#DBClusterSnapshot$Port */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.neptune#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_port(var_8);
            }
            ,
            s if s.matches("VpcId") /* VpcId com.amazonaws.neptune#DBClusterSnapshot$VpcId */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_9);
            }
            ,
            s if s.matches("ClusterCreateTime") /* ClusterCreateTime com.amazonaws.neptune#DBClusterSnapshot$ClusterCreateTime */ =>  {
                let var_10 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.neptune#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_cluster_create_time(var_10);
            }
            ,
            s if s.matches("MasterUsername") /* MasterUsername com.amazonaws.neptune#DBClusterSnapshot$MasterUsername */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_master_username(var_11);
            }
            ,
            s if s.matches("EngineVersion") /* EngineVersion com.amazonaws.neptune#DBClusterSnapshot$EngineVersion */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine_version(var_12);
            }
            ,
            s if s.matches("LicenseModel") /* LicenseModel com.amazonaws.neptune#DBClusterSnapshot$LicenseModel */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_license_model(var_13);
            }
            ,
            s if s.matches("SnapshotType") /* SnapshotType com.amazonaws.neptune#DBClusterSnapshot$SnapshotType */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_snapshot_type(var_14);
            }
            ,
            s if s.matches("PercentProgress") /* PercentProgress com.amazonaws.neptune#DBClusterSnapshot$PercentProgress */ =>  {
                let var_15 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.neptune#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_percent_progress(var_15);
            }
            ,
            s if s.matches("StorageEncrypted") /* StorageEncrypted com.amazonaws.neptune#DBClusterSnapshot$StorageEncrypted */ =>  {
                let var_16 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.neptune#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_storage_encrypted(var_16);
            }
            ,
            s if s.matches("KmsKeyId") /* KmsKeyId com.amazonaws.neptune#DBClusterSnapshot$KmsKeyId */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_id(var_17);
            }
            ,
            s if s.matches("DBClusterSnapshotArn") /* DBClusterSnapshotArn com.amazonaws.neptune#DBClusterSnapshot$DBClusterSnapshotArn */ =>  {
                let var_18 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_snapshot_arn(var_18);
            }
            ,
            s if s.matches("SourceDBClusterSnapshotArn") /* SourceDBClusterSnapshotArn com.amazonaws.neptune#DBClusterSnapshot$SourceDBClusterSnapshotArn */ =>  {
                let var_19 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_source_db_cluster_snapshot_arn(var_19);
            }
            ,
            s if s.matches("IAMDatabaseAuthenticationEnabled") /* IAMDatabaseAuthenticationEnabled com.amazonaws.neptune#DBClusterSnapshot$IAMDatabaseAuthenticationEnabled */ =>  {
                let var_20 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.neptune#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_iam_database_authentication_enabled(var_20);
            }
            ,
            s if s.matches("StorageType") /* StorageType com.amazonaws.neptune#DBClusterSnapshot$StorageType */ =>  {
                let var_21 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_type(var_21);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
