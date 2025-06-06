// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_key_material(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ImportKeyMaterial,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ImportKeyMaterial::RootCertificatePublicKey(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_2.key("RootCertificatePublicKey").start_object();
            crate::protocol_serde::shape_root_certificate_public_key::ser_root_certificate_public_key(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::ImportKeyMaterial::TrustedCertificatePublicKey(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_2.key("TrustedCertificatePublicKey").start_object();
            crate::protocol_serde::shape_trusted_certificate_public_key::ser_trusted_certificate_public_key(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::ImportKeyMaterial::Tr31KeyBlock(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_2.key("Tr31KeyBlock").start_object();
            crate::protocol_serde::shape_import_tr31_key_block::ser_import_tr31_key_block(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::ImportKeyMaterial::Tr34KeyBlock(inner) => {
            #[allow(unused_mut)]
            let mut object_4 = object_2.key("Tr34KeyBlock").start_object();
            crate::protocol_serde::shape_import_tr34_key_block::ser_import_tr34_key_block(&mut object_4, inner)?;
            object_4.finish();
        }
        crate::types::ImportKeyMaterial::KeyCryptogram(inner) => {
            #[allow(unused_mut)]
            let mut object_5 = object_2.key("KeyCryptogram").start_object();
            crate::protocol_serde::shape_import_key_cryptogram::ser_import_key_cryptogram(&mut object_5, inner)?;
            object_5.finish();
        }
        crate::types::ImportKeyMaterial::DiffieHellmanTr31KeyBlock(inner) => {
            #[allow(unused_mut)]
            let mut object_6 = object_2.key("DiffieHellmanTr31KeyBlock").start_object();
            crate::protocol_serde::shape_import_diffie_hellman_tr31_key_block::ser_import_diffie_hellman_tr31_key_block(&mut object_6, inner)?;
            object_6.finish();
        }
        crate::types::ImportKeyMaterial::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "ImportKeyMaterial",
            ))
        }
    }
    Ok(())
}
