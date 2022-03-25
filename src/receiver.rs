// This suppresses this warning, so the code can use the variable names
// as suggested in the document (`p_sender`, `P_receiver`).
#![allow(non_snake_case)]

use secp256k1::{SecretKey, PublicKey, Secp256k1, ecdh, hashes::{sha256, hmac, Hmac, Hash, HashEngine}};

use crate::constants;

/// Generate the master private key that can be used to derive addresses that will receive coins
pub fn generate_master_extended_private_key(
    P_change_sender: &PublicKey,
    sender_pub_key: &PublicKey,
    receiver_secret_key: &SecretKey) -> SecretKey {

    let secp = Secp256k1::new();

    // step 0
    // Select an input with lowest index, not belonging to the receiver,
    // being one of the types listed above - the sender key input.

    // step 1
    // Let P_sender be the public key associated with sender key input.
    let P_sender = sender_pub_key.clone();

    // step 2
    // Let p_receiver be the private key associated with Taproot address used by the receiver.
    let p_receiver = receiver_secret_key.clone();

    // step 3
    // Calculate shared_secret = SHA256(P_sender*p_receiver) (ECDH)
    let shared_secret = ecdh::SharedSecret::new(&P_sender, &p_receiver);

    // step 4
    // Calculte offset = HMAC(shared_secret, CHANGE_KEY_CONSTANT)
    let mut hmac_offset = hmac::HmacEngine::<sha256::Hash>::new(&constants::CHANGE_KEY_CONSTANT.to_be_bytes());
    hmac_offset.input(&shared_secret.secret_bytes());

    let offset = Hmac::<sha256::Hash>::from_engine(hmac_offset).into_inner();
    // step 5
    // Calculate P_change = offset*G + P_sender
    //let mut P_change = P_sender;
    let mut P_change = P_sender.clone();
    P_change.add_exp_assign(&secp, &offset).unwrap();

    // step 6
    // Check that P_change matches the key used in change.
    if P_change.cmp(P_change_sender) != std::cmp::Ordering::Equal {

        // step 7
        // If the key doesn't match, don't continue
        panic!("P_change keys don't match !");
    }

    // step 8
    // Calculate and securely cache relationship_seed = HMAC(shared_secret, RELATIONSHIP_SEED_CONSTANT)
    let mut hmac_relationship_seed = hmac::HmacEngine::<sha256::Hash>::new(&constants::RELATIONSHIP_SEED_CONSTANT.to_be_bytes());
    hmac_relationship_seed.input(&shared_secret.secret_bytes());

    let relationship_seed =  Hmac::<sha256::Hash>::from_engine(hmac_relationship_seed).into_inner();

    // step 9
    // Import the descriptor with the master private key p'_r = relationship_seed + p_reciever.
    let mut p_r = p_receiver.clone();
    p_r.add_assign(&relationship_seed).unwrap();

    p_r
}