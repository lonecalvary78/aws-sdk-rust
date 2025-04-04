// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_db_instance(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::DbInstance, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DbInstance::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBInstanceIdentifier") /* DBInstanceIdentifier com.amazonaws.neptune#DBInstance$DBInstanceIdentifier */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_instance_identifier(var_1);
            }
            ,
            s if s.matches("DBInstanceClass") /* DBInstanceClass com.amazonaws.neptune#DBInstance$DBInstanceClass */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_instance_class(var_2);
            }
            ,
            s if s.matches("Engine") /* Engine com.amazonaws.neptune#DBInstance$Engine */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine(var_3);
            }
            ,
            s if s.matches("DBInstanceStatus") /* DBInstanceStatus com.amazonaws.neptune#DBInstance$DBInstanceStatus */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_instance_status(var_4);
            }
            ,
            s if s.matches("MasterUsername") /* MasterUsername com.amazonaws.neptune#DBInstance$MasterUsername */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_master_username(var_5);
            }
            ,
            s if s.matches("DBName") /* DBName com.amazonaws.neptune#DBInstance$DBName */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_name(var_6);
            }
            ,
            s if s.matches("Endpoint") /* Endpoint com.amazonaws.neptune#DBInstance$Endpoint */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_endpoint::de_endpoint(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_endpoint(var_7);
            }
            ,
            s if s.matches("AllocatedStorage") /* AllocatedStorage com.amazonaws.neptune#DBInstance$AllocatedStorage */ =>  {
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
                builder = builder.set_allocated_storage(var_8);
            }
            ,
            s if s.matches("InstanceCreateTime") /* InstanceCreateTime com.amazonaws.neptune#DBInstance$InstanceCreateTime */ =>  {
                let var_9 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.neptune#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_instance_create_time(var_9);
            }
            ,
            s if s.matches("PreferredBackupWindow") /* PreferredBackupWindow com.amazonaws.neptune#DBInstance$PreferredBackupWindow */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_preferred_backup_window(var_10);
            }
            ,
            s if s.matches("BackupRetentionPeriod") /* BackupRetentionPeriod com.amazonaws.neptune#DBInstance$BackupRetentionPeriod */ =>  {
                let var_11 =
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
                builder = builder.set_backup_retention_period(var_11);
            }
            ,
            s if s.matches("DBSecurityGroups") /* DBSecurityGroups com.amazonaws.neptune#DBInstance$DBSecurityGroups */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_db_security_group_membership_list::de_db_security_group_membership_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_security_groups(var_12);
            }
            ,
            s if s.matches("VpcSecurityGroups") /* VpcSecurityGroups com.amazonaws.neptune#DBInstance$VpcSecurityGroups */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_vpc_security_group_membership_list::de_vpc_security_group_membership_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc_security_groups(var_13);
            }
            ,
            s if s.matches("DBParameterGroups") /* DBParameterGroups com.amazonaws.neptune#DBInstance$DBParameterGroups */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_db_parameter_group_status_list::de_db_parameter_group_status_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_parameter_groups(var_14);
            }
            ,
            s if s.matches("AvailabilityZone") /* AvailabilityZone com.amazonaws.neptune#DBInstance$AvailabilityZone */ =>  {
                let var_15 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_15);
            }
            ,
            s if s.matches("DBSubnetGroup") /* DBSubnetGroup com.amazonaws.neptune#DBInstance$DBSubnetGroup */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_db_subnet_group::de_db_subnet_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_subnet_group(var_16);
            }
            ,
            s if s.matches("PreferredMaintenanceWindow") /* PreferredMaintenanceWindow com.amazonaws.neptune#DBInstance$PreferredMaintenanceWindow */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_preferred_maintenance_window(var_17);
            }
            ,
            s if s.matches("PendingModifiedValues") /* PendingModifiedValues com.amazonaws.neptune#DBInstance$PendingModifiedValues */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_pending_modified_values::de_pending_modified_values(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_pending_modified_values(var_18);
            }
            ,
            s if s.matches("LatestRestorableTime") /* LatestRestorableTime com.amazonaws.neptune#DBInstance$LatestRestorableTime */ =>  {
                let var_19 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.neptune#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_latest_restorable_time(var_19);
            }
            ,
            s if s.matches("MultiAZ") /* MultiAZ com.amazonaws.neptune#DBInstance$MultiAZ */ =>  {
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
                builder = builder.set_multi_az(var_20);
            }
            ,
            s if s.matches("EngineVersion") /* EngineVersion com.amazonaws.neptune#DBInstance$EngineVersion */ =>  {
                let var_21 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine_version(var_21);
            }
            ,
            s if s.matches("AutoMinorVersionUpgrade") /* AutoMinorVersionUpgrade com.amazonaws.neptune#DBInstance$AutoMinorVersionUpgrade */ =>  {
                let var_22 =
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
                builder = builder.set_auto_minor_version_upgrade(var_22);
            }
            ,
            s if s.matches("ReadReplicaSourceDBInstanceIdentifier") /* ReadReplicaSourceDBInstanceIdentifier com.amazonaws.neptune#DBInstance$ReadReplicaSourceDBInstanceIdentifier */ =>  {
                let var_23 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_read_replica_source_db_instance_identifier(var_23);
            }
            ,
            s if s.matches("ReadReplicaDBInstanceIdentifiers") /* ReadReplicaDBInstanceIdentifiers com.amazonaws.neptune#DBInstance$ReadReplicaDBInstanceIdentifiers */ =>  {
                let var_24 =
                    Some(
                        crate::protocol_serde::shape_read_replica_db_instance_identifier_list::de_read_replica_db_instance_identifier_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_read_replica_db_instance_identifiers(var_24);
            }
            ,
            s if s.matches("ReadReplicaDBClusterIdentifiers") /* ReadReplicaDBClusterIdentifiers com.amazonaws.neptune#DBInstance$ReadReplicaDBClusterIdentifiers */ =>  {
                let var_25 =
                    Some(
                        crate::protocol_serde::shape_read_replica_db_cluster_identifier_list::de_read_replica_db_cluster_identifier_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_read_replica_db_cluster_identifiers(var_25);
            }
            ,
            s if s.matches("LicenseModel") /* LicenseModel com.amazonaws.neptune#DBInstance$LicenseModel */ =>  {
                let var_26 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_license_model(var_26);
            }
            ,
            s if s.matches("Iops") /* Iops com.amazonaws.neptune#DBInstance$Iops */ =>  {
                let var_27 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.neptune#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_iops(var_27);
            }
            ,
            s if s.matches("OptionGroupMemberships") /* OptionGroupMemberships com.amazonaws.neptune#DBInstance$OptionGroupMemberships */ =>  {
                let var_28 =
                    Some(
                        crate::protocol_serde::shape_option_group_membership_list::de_option_group_membership_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_option_group_memberships(var_28);
            }
            ,
            s if s.matches("CharacterSetName") /* CharacterSetName com.amazonaws.neptune#DBInstance$CharacterSetName */ =>  {
                let var_29 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_character_set_name(var_29);
            }
            ,
            s if s.matches("SecondaryAvailabilityZone") /* SecondaryAvailabilityZone com.amazonaws.neptune#DBInstance$SecondaryAvailabilityZone */ =>  {
                let var_30 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_secondary_availability_zone(var_30);
            }
            ,
            s if s.matches("PubliclyAccessible") /* PubliclyAccessible com.amazonaws.neptune#DBInstance$PubliclyAccessible */ =>  {
                let var_31 =
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
                builder = builder.set_publicly_accessible(var_31);
            }
            ,
            s if s.matches("StatusInfos") /* StatusInfos com.amazonaws.neptune#DBInstance$StatusInfos */ =>  {
                let var_32 =
                    Some(
                        crate::protocol_serde::shape_db_instance_status_info_list::de_db_instance_status_info_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status_infos(var_32);
            }
            ,
            s if s.matches("StorageType") /* StorageType com.amazonaws.neptune#DBInstance$StorageType */ =>  {
                let var_33 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_type(var_33);
            }
            ,
            s if s.matches("TdeCredentialArn") /* TdeCredentialArn com.amazonaws.neptune#DBInstance$TdeCredentialArn */ =>  {
                let var_34 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_tde_credential_arn(var_34);
            }
            ,
            s if s.matches("DbInstancePort") /* DbInstancePort com.amazonaws.neptune#DBInstance$DbInstancePort */ =>  {
                let var_35 =
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
                builder = builder.set_db_instance_port(var_35);
            }
            ,
            s if s.matches("DBClusterIdentifier") /* DBClusterIdentifier com.amazonaws.neptune#DBInstance$DBClusterIdentifier */ =>  {
                let var_36 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_identifier(var_36);
            }
            ,
            s if s.matches("StorageEncrypted") /* StorageEncrypted com.amazonaws.neptune#DBInstance$StorageEncrypted */ =>  {
                let var_37 =
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
                builder = builder.set_storage_encrypted(var_37);
            }
            ,
            s if s.matches("KmsKeyId") /* KmsKeyId com.amazonaws.neptune#DBInstance$KmsKeyId */ =>  {
                let var_38 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_id(var_38);
            }
            ,
            s if s.matches("DbiResourceId") /* DbiResourceId com.amazonaws.neptune#DBInstance$DbiResourceId */ =>  {
                let var_39 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_dbi_resource_id(var_39);
            }
            ,
            s if s.matches("CACertificateIdentifier") /* CACertificateIdentifier com.amazonaws.neptune#DBInstance$CACertificateIdentifier */ =>  {
                let var_40 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ca_certificate_identifier(var_40);
            }
            ,
            s if s.matches("DomainMemberships") /* DomainMemberships com.amazonaws.neptune#DBInstance$DomainMemberships */ =>  {
                let var_41 =
                    Some(
                        crate::protocol_serde::shape_domain_membership_list::de_domain_membership_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_domain_memberships(var_41);
            }
            ,
            s if s.matches("CopyTagsToSnapshot") /* CopyTagsToSnapshot com.amazonaws.neptune#DBInstance$CopyTagsToSnapshot */ =>  {
                let var_42 =
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
                builder = builder.set_copy_tags_to_snapshot(var_42);
            }
            ,
            s if s.matches("MonitoringInterval") /* MonitoringInterval com.amazonaws.neptune#DBInstance$MonitoringInterval */ =>  {
                let var_43 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.neptune#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_monitoring_interval(var_43);
            }
            ,
            s if s.matches("EnhancedMonitoringResourceArn") /* EnhancedMonitoringResourceArn com.amazonaws.neptune#DBInstance$EnhancedMonitoringResourceArn */ =>  {
                let var_44 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_enhanced_monitoring_resource_arn(var_44);
            }
            ,
            s if s.matches("MonitoringRoleArn") /* MonitoringRoleArn com.amazonaws.neptune#DBInstance$MonitoringRoleArn */ =>  {
                let var_45 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_monitoring_role_arn(var_45);
            }
            ,
            s if s.matches("PromotionTier") /* PromotionTier com.amazonaws.neptune#DBInstance$PromotionTier */ =>  {
                let var_46 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.neptune#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_promotion_tier(var_46);
            }
            ,
            s if s.matches("DBInstanceArn") /* DBInstanceArn com.amazonaws.neptune#DBInstance$DBInstanceArn */ =>  {
                let var_47 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_instance_arn(var_47);
            }
            ,
            s if s.matches("Timezone") /* Timezone com.amazonaws.neptune#DBInstance$Timezone */ =>  {
                let var_48 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_timezone(var_48);
            }
            ,
            s if s.matches("IAMDatabaseAuthenticationEnabled") /* IAMDatabaseAuthenticationEnabled com.amazonaws.neptune#DBInstance$IAMDatabaseAuthenticationEnabled */ =>  {
                let var_49 =
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
                builder = builder.set_iam_database_authentication_enabled(var_49);
            }
            ,
            s if s.matches("PerformanceInsightsEnabled") /* PerformanceInsightsEnabled com.amazonaws.neptune#DBInstance$PerformanceInsightsEnabled */ =>  {
                let var_50 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.neptune#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_performance_insights_enabled(var_50);
            }
            ,
            s if s.matches("PerformanceInsightsKMSKeyId") /* PerformanceInsightsKMSKeyId com.amazonaws.neptune#DBInstance$PerformanceInsightsKMSKeyId */ =>  {
                let var_51 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_performance_insights_kms_key_id(var_51);
            }
            ,
            s if s.matches("EnabledCloudwatchLogsExports") /* EnabledCloudwatchLogsExports com.amazonaws.neptune#DBInstance$EnabledCloudwatchLogsExports */ =>  {
                let var_52 =
                    Some(
                        crate::protocol_serde::shape_log_type_list::de_log_type_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_enabled_cloudwatch_logs_exports(var_52);
            }
            ,
            s if s.matches("DeletionProtection") /* DeletionProtection com.amazonaws.neptune#DBInstance$DeletionProtection */ =>  {
                let var_53 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.neptune#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_deletion_protection(var_53);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
