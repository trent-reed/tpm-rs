# tpm2-rs-errors

Definition for the error codes for TPMs.

## TpmError

This represents an abstract TPM error, it is defined in accordance to the TPM2
specification. The error code for TPMs are quite structured, so there's a lot of
helper functionality here to help inspect the error.

Even though TPM error codes are 32 bits, practically speaking the TPM can only
ever use the first 12 of those bits. The reason for that is because they must be
compatible with TSS error codes (read more on those below).

## TssError

While TPMs error codes are sensible for sending directly to/from a TPM, there's
more that can go wrong in the stack other than just the TPM failing. TSS errors
are where that is accounted for.

Simply-put, the upper 20 bits are reserved by the TSS specification, and they
provide a few different "error layers". The 0th layer is the TPM layer (this is
why the upper 20 bits of a TPM error must always be 0).

It is correct to say that all TPM errors are TSS errors, but you cannot say the
other way around (because the other error layers will set values in that upper
20-bits to suggest that it is not a TPM error).
