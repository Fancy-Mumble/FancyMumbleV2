use std::error::Error;

use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    pkey::PKey,
    rsa::Rsa,
    x509::{
        extension::{BasicConstraints, KeyUsage},
        X509NameBuilder, X509,
    },
};
use tokio_native_tls::native_tls::{self, Identity};

struct CertificateStore {
    certificate: Vec<u8>,
    private_key: Vec<u8>,
}

async fn create_tls_certificate() -> Result<CertificateStore, Box<dyn Error>> {
    //TODO: Currently we always generate a new certificate. We should store the certificate and private key in a file and only generate a new one if the file does not exist.
    //TODO: We should also check if the certificate is still valid and generate a new one if it is not.
    //TODO: We currently always use fancy-mumble.com as the certificate's common name. We should use a client specified common name instead.

    let rsa = Rsa::generate(2048)?;
    let private_key = PKey::from_rsa(rsa)?;

    let mut x509 = X509::builder()?;
    let mut name = X509NameBuilder::new()?;
    name.append_entry_by_text("CN", "fancy-mumble.com")?;
    let name = name.build();
    x509.set_subject_name(&name)?;
    x509.set_issuer_name(&name)?;
    x509.set_pubkey(&private_key)?;

    let not_before = Asn1Time::days_from_now(0)?;
    let not_after = Asn1Time::days_from_now(365 * 10)?;
    x509.set_not_before(&not_before)?;
    x509.set_not_after(&not_after)?;

    let basic_constraints = BasicConstraints::new().critical().ca().build()?;
    x509.append_extension(basic_constraints)?;
    let key_usage = KeyUsage::new()
        .digital_signature()
        .key_encipherment()
        .build()?;
    x509.append_extension(key_usage)?;
    x509.sign(&private_key, MessageDigest::sha256())?;

    let crt = x509.build().to_pem()?;
    let pkey = private_key.private_key_to_pem_pkcs8()?;

    Ok(CertificateStore {
        certificate: crt,
        private_key: pkey,
    })
}

pub async fn get_client_certificate() -> Result<Identity, Box<dyn Error>> {
    let cert_store = create_tls_certificate().await?;

    let identity = native_tls::Identity::from_pkcs8(&cert_store.certificate, &cert_store.private_key)?;

    return Ok(identity);
}
