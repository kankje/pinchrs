use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::prelude::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(path: &str, signature: &str, key: &str) -> anyhow::Result<()> {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())?;
    mac.update(path.as_bytes());

    let bytes = BASE64_URL_SAFE_NO_PAD.decode(signature.replace('=', ""))?;
    mac.verify_slice(&bytes)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifies_signature() {
        let result = verify_signature(
            "path",
            "3l4QxaOncm-DcnDXBNuIfhuU2n09m7P3gHizQ2Bvf9E",
            "secret",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_fails_to_verify_signature() {
        let result = verify_signature("path", "invalid", "secret");
        assert!(result.is_err());
    }
}
