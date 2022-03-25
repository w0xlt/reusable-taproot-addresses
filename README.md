# Reusable Taproot Addresses

## Documentation

This work is still in progress. Do not use this library on mainnet.

Currently it implements the steps described in [Inital Transaction](https://gist.github.com/Kixunil/0ddb3a9cdec33342b97431e438252c0a?permalink_comment_id=4081736#initial-transaction) section of "Reusable taproot addresses" document.

But instead of calculating the addresses, both sender and receiver functions return the key that can be used to derive the addresses. This was suggested in [this comment](https://gist.github.com/Kixunil/0ddb3a9cdec33342b97431e438252c0a?permalink_comment_id=4108087#gistcomment-4108087).

Sender function (`generate_master_extended_public_key`) returns a public key, which must be used as a master extendend public key to derive all addresses to send coins.

Receiver function (`generate_master_extended_private_key`) returns a private key, which must be used as a master extendend private key to derive all addresses to receive coins.

1) The first step is to calculate the `change address` of the sender and the `relationship_seed` that the sender will share with the receiver.

```rust
pub fn sender::generate_change_output_script(sender_secret_key: &SecretKey, receiver_pub_key: &PublicKey) -> (PublicKey, [u8; 32]);
```

The `sender_secret_key` must be the private key associated with the input with the lowest index, belonging to the sender.

The `receiver_pub_key` must be the public key of the Taproot address used by the receiver.

This function returns a tuple. The first element is the `PublicKey` of the change address and the second is the 32-byte relationship seed (`[u8; 32]`).


2) The sender can now calculate the addresses that can be used to send funds to the recipient.

```rust
pub fn sender::generate_master_extended_public_key(relationship_seed: &[u8; 32], receiver_pub_key: &PublicKey) -> Vec<PublicKey>;
```

The `relationship_seed` was calculated in the first step and must be used to derive the public keys.

The `receiver_pub_key` must be the public key of the Taproot address used by the receiver.

The function returns a list of public keys (`PublicKey`) that will be used as a master public key to derive the addresses to send coins.

3) Likewise, the receiver can calculate the same addresses, watches them and react to the received transaction.

```rust
pub fn receiver::generate_master_extended_private_key(P_change_sender: &PublicKey, sender_pub_key: &PublicKey, receiver_secret_key: &SecretKey) -> Vec<PublicKey>;
```

The `P_change_sender` must be the public key of the `change address` of the sender calculated in the fist step (the first element of the tuple returned by the `sender::generate_change_output_script` function).

The `sender_pub_key` must be the public key associated with the input with the lowest index, belonging to the sender.

The `receiver_secret_key` must be the private key of the Taproot address used by the receiver.

The function returns a list of public keys (`SecretKey`) that will be used as a master private key to derive all addresses to receive coins.

| :exclamation:  The public key returned by `sender::generate_master_extended_public_key` must be the same as the one calculated from the private key returned by `receiver::generate_master_extended_private_key`.  |
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
```
