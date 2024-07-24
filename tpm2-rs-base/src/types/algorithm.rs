// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// TYPE
// =============================================================================

/// TpmaAlgorithm defines the attributes of an algorithm (TPMA_ALGORITHM).
/// See definition in Part 2: Structures, section 8.2.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Marshalable)]
pub struct TpmaAlgorithm(pub u32);

// =============================================================================
// IMPLEMENTATION
// =============================================================================

// -----------------------------------------------------------------------------
bitflags! {
    impl TpmaAlgorithm : u32 {
        /// Indicates an asymmetric algorithm with public and private portions.
        const ASYMMETRIC = 1 << 0;
        /// Indicates a symmetric block cipher.
        const SYMMETRIC = 1 << 1;
        /// Indicates a hash algorithm.
        const HASH = 1 << 2;
        /// Indicates an algorithm that may be used as an object type.
        const OBJECT = 1 << 3;
        /// Indicates a signing algorithm.
        const SIGNING = 1 << 8;
        /// Indicates an encryption/decryption algorithm.
        const ENCRYPTING = 1 << 9;
        /// Indicates a method such as a key derivative function.
        const METHOD = 1 << 10;
    }
}
