use sui_keys::keystore::{AccountKeystore, InMemKeystore};
use sui_sdk::types::{
    base_types::{ObjectID, ObjectRef, SequenceNumber},
    crypto::{EncodeDecodeBase64, SuiKeyPair},
};

// let keystore = sui_keys::keystore::FileBasedKeystore::new(&std::path::Path::new("/root/.sui/sui_config/sui.keystore").to_path_buf()).unwrap();
pub fn get_keystore() -> InMemKeystore {
    let mut keystore = InMemKeystore::default();
    let home = std::env::var("HOME").unwrap();
    let keys = std::fs::read_to_string(format!("{home}/.sui/sui_config/sui.keystore")).unwrap();
    let private_keys: Vec<String> = serde_json::from_str(&keys).unwrap();
    let private_key = SuiKeyPair::decode_base64(&private_keys[0]).unwrap();
    keystore.add_key(None, private_key).unwrap();
    keystore
}
#[test]
fn test_get_keystore() {
    get_keystore();
}

/*
sui_client.coin_read_api().get_coins(sender, None, None, None).await.unwrap().data.first().unwrap()

ObjectDigest Debug and Display different!
Debug: o#322w2iikuU3NLigkiDzAAt7NDdDF3iUGHUMx3khWGzV4
Display: 322w2iikuU3NLigkiDzAAt7NDdDF3iUGHUMx3khWGzV4
*/
pub fn gas_coin_obj_ref(is_testnet: bool) -> ObjectRef {
    let (object_id, version, digest) = if is_testnet {
        (
            "0xb5b3c003ccf358add5caa6954e62b5c1d27ee6c509563a140189eee5d8b41dbd",
            627549,
            "322w2iikuU3NLigkiDzAAt7NDdDF3iUGHUMx3khWGzV4",
        )
    } else {
        (
            "0x320be440ed3cde4a7dcb05e25b114824039d14201adc9b205ecc6eaf1e57d009",
            295741243,
            "eDHmXKDX4xqay435zx1LFsct3rwaEjLzbXwBnpCymrZ",
        )
    };
    (
        ObjectID::from_hex_literal(object_id).expect(object_id),
        SequenceNumber::from_u64(version),
        digest.parse().expect(digest),
    )
}
#[test]
fn test_gas_coin_obj_ref() {
    gas_coin_obj_ref(true);
    gas_coin_obj_ref(false);
}
