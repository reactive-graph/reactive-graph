use crate::tooling::instances::certificates::args::GenerateCertificateArgs;
use rcgen::Ia5String;
use rcgen::SanType;
use rustls_cert_gen::CertificateBuilder;
use std::path::Path;
use std::path::PathBuf;

pub mod args;

pub const KEYS_DIR_NAME: &str = "keys";

pub const DEFAULT_CERTIFICATE_FILE_NAME: &str = "cert";
pub const DEFAULT_CA_FILE_NAME: &str = "ca";
pub const PUBLIC_KEY_FILE_NAME: &str = "cert.pem";

pub const PRIVATE_KEY_FILE_NAME: &str = "cert.key.pem";

pub fn handle_generate_certificate(instance_dir: &Path, args: GenerateCertificateArgs) -> anyhow::Result<()> {
    let keys_dir = get_keys_dir(instance_dir);
    generate_certificate(&keys_dir, args)?;
    Ok(())
}

pub fn generate_certificate(keys_dir: &Path, args: GenerateCertificateArgs) -> anyhow::Result<()> {
    let country_name = args.country_name.unwrap_or(String::from("de"));
    let organization_name = args.organization_name.unwrap_or(String::from("Reactive Graph"));
    let common_name = args.common_name.unwrap_or(String::from("localhost"));

    let ca = CertificateBuilder::new()
        .certificate_authority()
        .country_name(&country_name)?
        .organization_name(&organization_name)
        .build()?;
    let subject_alt_names = vec![SanType::DnsName(Ia5String::try_from(common_name.clone())?)];
    CertificateBuilder::new()
        .end_entity()
        .common_name(&common_name)
        .subject_alternative_names(subject_alt_names)
        .build(&ca)?
        .serialize_pem()
        .write(keys_dir, DEFAULT_CERTIFICATE_FILE_NAME)?;
    ca.serialize_pem().write(keys_dir, DEFAULT_CA_FILE_NAME)?;
    Ok(())
}

pub fn get_keys_dir(instance_dir: &Path) -> PathBuf {
    let mut keys_dir = instance_dir.to_owned();
    keys_dir.push(KEYS_DIR_NAME);
    keys_dir
}
