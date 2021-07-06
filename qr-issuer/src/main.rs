use qrcode::QrCode;
use image::Luma;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::env;

fn main() {
    type HmacSha256 = Hmac<Sha256>;

    let secret = match env::var("SECRET") {
        Ok(val) => val,
        Err(_e) => panic!("No secret set"),
    };

    let message = b"1625614920";

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();

    mac.update(message);

    let hash_message = mac.finalize().into_bytes();

    // to base64
    let result = base64::encode(hash_message);

    let code = QrCode::new(&result).unwrap();

    println!("{}", &result);

    let image = code.render::<Luma<u8>>().build();

    image.save("/tmp/qrcode.png").unwrap();
}
