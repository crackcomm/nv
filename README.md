# nv

Secure password store highly resistant to brute-force attacks.

Implements ideas from both blockchain and proof of work for creating a password repository and hashing repository password.


# mnemonic

In contrary to a combination of password and mnemonic, if you remember your password it is possible to brute-force the mnemonic by design.

If you don't want to be able to brute-force the mnemonic just use more seed bytes with `-b` flag.

Seed possibilites are `256^b` where `b` is amount of seed bytes.
Using one additional byte brings security up significantly.

It is safe enough to use four seed bytes with three seed words but nearly impossible to cheaply brute-force in case of loss.

# security

Security is a combination of parameters: `difficulty`, `round` and `seed-bytes`.

It is possible to cleverly manage those in order to make it harder to brute-force and faster to use at the same time.

# encryption

Uses [zbox](https://zbox.io/) file system with `Cipher::Xchacha`.

# cloud storage

It is possible to store your password repository in [ZboxFS](https://zbox.io/fs/) cloud.

# guarantees

* Zero-knowledge
* Zero-guarantees (backup your password repository)
