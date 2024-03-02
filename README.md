# Bun Crypto

A file encryption library for Node.js/Bun and backed by native Rust speed - uses the `age` Rust crate under the hood.

Currently only works with GNU/Linux x64 and MSVC Windows x64 targets.

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

// Generate a SHA512 hash string when given -
// file data of type `Uint8Array`
genFileHash(fileData: Uint8Array): string
```