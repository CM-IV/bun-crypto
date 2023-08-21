# Bun Crypto

A library for `Node.js` and `Bun` that uses `napi` and the `age` Rust encryption crate.

## Exported Functions

```ts
// Generate an `age::x25519::Identity` secret key
genSecretKey(): string

// Derive the `age::x25519::Recipient` public key -
// when given the secret key string
derivePubKey(secret: string): string

// Encrypt binary data of type `Uint8Array` when also -
// given the public key string
encryptFile(fileData: Uint8Array, pubStr: string): Uint8Array

// Decrypt binary data of an encrypted file with -
// type `Uint8Array` when also provided the secret key string
decryptFile(encryptedData: Uint8Array, secretKeyStr: string): Uint8Array
```