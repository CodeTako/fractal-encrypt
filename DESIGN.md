# Overview

This is meant to be fun and better than nothing, but should not be considered super secure. I don't know enough about cryptography to make any claims around that. That said I hope that it will be reasonably secure given the large key and the math used.

The idea is to use the simple math of julia sets to generate an unpredictable stream of data to use for encryption.

Most of the data will be used to XOR with the plaintext, and some small chunks will be used for added befuddling with shuffling or added noise.

The key will consist of:
- the complex number c = x + yi, where x and y are between -2:2
- Optionally a starting point z = a + bi, where a and b are between -2:2
If each number in the key is represented by 128 bits, the key will be 256 or 512 bits.

The update step is z_i+1 = z^2 + c which turns into z_i+1 = a^2 - b^2 + 2abi + x + yi

If z for any step "escapes" from the valid range, it will be returned to the range by wrapping it:
- If x < -2 then x = x + 4 
- If x > 2 them x = x - 4 
- Repeat if necessary

The algorithm starts with 12 updates, and then from there each step will be used to encrypt a chunk of plaintext into an encrypted text.
1. XOR with z
2. Use the final digits of z to shuffle the bytes
3. Add 0-3 bytes of noise, based on the value of set bits in z

Encrypted file will be LARGER than the plaintext. This algorithm will likely be much slower than AES or other more common algorithms.

But fractals are cool!
