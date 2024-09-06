use impl_serde::serialize::to_hex;
use ss58_registry::Ss58AddressFormat;
use crate::{Pair, sr25519};
use crate::crypto::Ss58Codec;
use crate::sr25519::Public;

#[test]
pub fn get_address() {

    // let p = sr25519::Pair::from_string(&"fiscal document grain ecology wheat around sport nice guitar topple add north".to_string(), None).unwrap();
    // let public_key = &p.public().to_string();
    // println!("public_key = {}",&public_key);
    let seed = sr25519::Pair::from_phrase(&"fiscal document grain ecology wheat around sport nice guitar topple add north".to_string(), None).unwrap();
    let private_key = &seed.0.seed();
    let value = to_hex(&private_key.to_bytes()[0..32], false);

    println!("seed = {}", to_hex(seed.1.as_ref(), false));


    println!("private_key = {}", value);
    let public_key = &private_key.to_public();
    println!("public_key = {}", to_hex(&public_key.to_bytes(), false));
    println!("address = {}", &seed.0.public().to_ss58check_with_version(Ss58AddressFormat::from(0u16)));
}