/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

//! Functions to create signing keys and calculate signatures.

use crate::date_time::format_date;

#[cfg(feature = "ring")]
use ring::{
	hmac::{self, Key, Tag}
};

#[cfg(feature = "openssl")]
use openssl::{
	hash::MessageDigest,
	pkey::PKey,
	sign::Signer
};

use std::time::SystemTime;

/// HashedPayload = Lowercase(HexEncode(Hash(requestPayload)))
#[cfg(feature = "openssl")]
#[allow(dead_code)] // Unused when compiling without certain features
pub(crate) fn sha256_hex_string(bytes: impl AsRef<[u8]>) -> String {
	use openssl::sha::sha256;
    // hex::encode returns a lowercase string
	hex::encode(sha256(bytes.as_ref()))
}

/// HashedPayload = Lowercase(HexEncode(Hash(requestPayload)))
#[cfg(feature = "ring")]
#[allow(dead_code)] // Unused when compiling without certain features
pub(crate) fn sha256_hex_string(bytes: impl AsRef<[u8]>) -> String {
	use ring::digest;
    // hex::encode returns a lowercase string
	hex::encode(digest::digest(&digest::SHA256, bytes.as_ref()))
}

/// Calculates a Sigv4 signature
#[cfg(feature = "ring")]
pub fn calculate_signature(signing_key: Tag, string_to_sign: &[u8]) -> String {
    let s_key = Key::new(hmac::HMAC_SHA256, signing_key.as_ref());
    let tag = hmac::sign(&s_key, string_to_sign);
    hex::encode(tag)
}

/// Generates a signing key for Sigv4
#[cfg(feature = "ring")]
pub fn generate_signing_key(
    secret: &str,
    time: SystemTime,
    region: &str,
    service: &str,
) -> hmac::Tag {
    // kSecret = your secret access key
    // kDate = HMAC("AWS4" + kSecret, Date)
    // kRegion = HMAC(kDate, Region)
    // kService = HMAC(kRegion, Service)
    // kSigning = HMAC(kService, "aws4_request")

    let secret = format!("AWS4{}", secret);
    let secret = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&secret, format_date(time).as_bytes());

    // sign region
    let key = hmac::Key::new(hmac::HMAC_SHA256, tag.as_ref());
    let tag = hmac::sign(&key, region.as_bytes());

    // sign service
    let key = hmac::Key::new(hmac::HMAC_SHA256, tag.as_ref());
    let tag = hmac::sign(&key, service.as_bytes());

    // sign request
    let key = hmac::Key::new(hmac::HMAC_SHA256, tag.as_ref());
    hmac::sign(&key, "aws4_request".as_bytes())
}

#[cfg(feature = "openssl")]
fn sign_sha256(secret: &[u8], buf_to_sign: &[u8]) -> Vec<u8> {
	let key = PKey::hmac(secret).unwrap();
	let mut signer = Signer::new(MessageDigest::sha256(), &key).unwrap();
	signer.update(buf_to_sign).unwrap();
	signer.sign_to_vec().unwrap()
}

/// Calculates a Sigv4 signature
#[cfg(feature = "openssl")]
pub fn calculate_signature(signing_key: Vec<u8>, string_to_sign: &[u8]) -> String {
	let signature = sign_sha256(signing_key.as_slice(), string_to_sign);
    hex::encode(signature)
}

/// Generates a signing key for Sigv4
#[cfg(feature = "openssl")]
pub fn generate_signing_key(
    secret: &str,
    time: SystemTime,
    region: &str,
    service: &str,
) -> Vec<u8> {
    // kSecret = your secret access key
    // kDate = HMAC("AWS4" + kSecret, Date)
    // kRegion = HMAC(kDate, Region)
    // kService = HMAC(kRegion, Service)
    // kSigning = HMAC(kService, "aws4_request")

    let secret = format!("AWS4{}", secret);
	let signature = sign_sha256(secret.as_bytes(), format_date(time).as_bytes());
	let signature = sign_sha256(signature.as_slice(), region.as_bytes());
	let signature = sign_sha256(signature.as_slice(), service.as_bytes());
	sign_sha256(signature.as_slice(), "aws4_request".as_bytes())
}

#[cfg(test)]
mod tests {
    use super::{calculate_signature, generate_signing_key};
    use crate::date_time::test_parsers::parse_date_time;
    use crate::http_request::test::test_canonical_request;
    use crate::sign::sha256_hex_string;

    #[test]
    fn test_signature_calculation() {
        let secret = "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY";
        let creq = test_canonical_request("iam");
        let time = parse_date_time("20150830T123600Z").unwrap();

        let derived_key = generate_signing_key(secret, time, "us-east-1", "iam");
        let signature = calculate_signature(derived_key, creq.as_bytes());

        let expected = "5d672d79c15b13162d9279b0855cfba6789a8edb4c82c400e06b5924a6f2b5d7";
        assert_eq!(expected, &signature);
    }

    #[test]
    fn sign_payload_empty_string() {
        let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let actual = sha256_hex_string(&[]);
        assert_eq!(expected, actual);
    }
}