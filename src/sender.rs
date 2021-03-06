// This suppresses this warning, so the code can use the variable names
// as suggested in the document (`p_sender`, `P_receiver`).
#![allow(non_snake_case)]

use secp256k1::{SecretKey, PublicKey, Secp256k1, ecdh, hashes::{sha256, hmac, Hmac, Hash, HashEngine}};

use crate::constants;

/// Generate the change output script (public key) and the relationship seed.
/// Both must be shared with the receiver.
pub fn generate_change_output_script(
    sender_secret_key: &SecretKey,
    receiver_pub_key: &PublicKey) -> (PublicKey, [u8; 32]) {

    let secp = Secp256k1::new();

    // PART 01: generate a change

    // step 0
    // suppose that sender_secret_key is the private key associated with sender key input.

    // step 1
    // Let p_sender be the private key associated with sender key input.
    let mut p_sender = sender_secret_key.clone();

    // step 2
    // Let P_receiver be the Taproot address used by the receiver.
    let P_receiver = receiver_pub_key.clone();

    // step 3
    // Calculate shared_secret = SHA256(p_sender*P_receiver) (ECDH)
    let shared_secret = ecdh::SharedSecret::new(&P_receiver, &p_sender);

    // step 4
    // Calculte offset = HMAC(shared_secret, CHANGE_KEY_CONSTANT) where CHANGE_KEY_CONSTANT
    // is an arbitrary constant defined by the protocol
    let mut hmac_offset = hmac::HmacEngine::<sha256::Hash>::new(&constants::CHANGE_KEY_CONSTANT.to_be_bytes());
    hmac_offset.input(&shared_secret.secret_bytes());

    let offset = Hmac::<sha256::Hash>::from_engine(hmac_offset).into_inner();

    // step 5
    // Calculate P_change = (offset + p_sender)*G
    let _  = p_sender.add_assign(&offset);
    let P_change = PublicKey::from_secret_key(&secp, &p_sender);

    // step 6
    // Calculate and securely cache relationship_seed = HMAC(shared_secret, RELATIONSHIP_SEED_CONSTANT)
    let mut hmac_relationship_seed = hmac::HmacEngine::<sha256::Hash>::new(&constants::RELATIONSHIP_SEED_CONSTANT.to_be_bytes());
    hmac_relationship_seed.input(&shared_secret.secret_bytes());

    let relationship_seed =  Hmac::<sha256::Hash>::from_engine(hmac_relationship_seed).into_inner();

    // step 7
    // Use P_change in the change output script
    (P_change, relationship_seed)

}

/// Generate the master extended public key that can be used to derive addresses to send funds to the receiver
pub fn generate_master_extended_public_key(
    relationship_seed: &[u8; 32],
    receiver_pub_key: &PublicKey) -> PublicKey {

    let secp = Secp256k1::new();

    // Sender can compute the xpub using P'r = P_reciever + relationship_seed*G.
    let relationship_seed_secret_key = SecretKey::from_slice(relationship_seed).unwrap();
    //let relationship_seed_pulic_key = PublicKey::from_secret_key(&secp, &relationship_seed_secret_key);
    let mut P_r = receiver_pub_key.clone();
    P_r.add_exp_assign(&secp, &relationship_seed_secret_key.secret_bytes()).unwrap();

    P_r
}