use super::*;

// TPM2PT represents a TPM_PT.
// See definition in Part 2: Structures, section 6.13.
#[open_enum]
#[repr(u32)]
#[rustfmt::skip] #[derive(Debug)] // Keep debug derivation separate for open_enum override.
#[derive(Copy, Clone, Default, Marshalable)]
pub enum TPM2PT {
    // a 4-octet character string containing the TPM Family value
    // (TPM_SPEC_FAMILY)
    FamilyIndicator = 0x00000100,
    // the level of the specification
    Level = 0x00000101,
    // the specification Revision times 100
    Revision = 0x00000102,
    // the specification day of year using TCG calendar
    DayofYear = 0x00000103,
    // the specification year using the CE
    Year = 0x00000104,
    // the vendor ID unique to each TPM manufacturer
    Manufacturer = 0x00000105,
    // the first four characters of the vendor ID string
    VendorString1 = 0x00000106,
    // the second four characters of the vendor ID string
    VendorString2 = 0x00000107,
    // the third four characters of the vendor ID string
    VendorString3 = 0x00000108,
    // the fourth four characters of the vendor ID sting
    VendorString4 = 0x00000109,
    // vendor-defined value indicating the TPM model
    VendorTPMType = 0x0000010A,
    // the most-significant 32 bits of a TPM vendor-specific value
    // indicating the version number of the firmware.
    FirmwareVersion1 = 0x0000010B,
    // the least-significant 32 bits of a TPM vendor-specific value
    // indicating the version number of the firmware.
    FirmwareVersion2 = 0x0000010C,
    // the maximum size of a parameter TPM2B_MAX_BUFFER)
    InputBuffer = 0x0000010D,
    // the minimum number of transient objects that can be held in TPM RAM
    HRTransientMin = 0x0000010E,
    // the minimum number of persistent objects that can be held in TPM NV
    // memory
    HRPersistentMin = 0x0000010F,
    // the minimum number of authorization sessions that can be held in TPM
    // RAM
    HRLoadedMin = 0x00000110,
    // the number of authorization sessions that may be active at a time
    ActiveSessionsMax = 0x00000111,
    // the number of PCR implemented
    PCRCount = 0x00000112,
    // the minimum number of octets in a TPMS_PCR_SELECT.sizeOfSelect
    PCRSelectMin = 0x00000113,
    // the maximum allowed difference (unsigned) between the contextID
    // values of two saved session contexts
    ContextGapMax = 0x00000114,
    // the maximum number of NV Indexes that are allowed to have the
    // TPM_NT_COUNTER attribute
    NVCountersMax = 0x00000116,
    // the maximum size of an NV Index data area
    NVIndexMax = 0x00000117,
    // a TPMA_MEMORY indicating the memory management method for the TPM
    Memory = 0x00000118,
    // interval, in milliseconds, between updates to the copy of
    // TPMS_CLOCK_INFO.clock in NV
    ClockUpdate = 0x00000119,
    // the algorithm used for the integrity HMAC on saved contexts and for
    // hashing the fuData of TPM2_FirmwareRead()
    ContextHash = 0x0000011A,
    // TPM_ALG_ID, the algorithm used for encryption of saved contexts
    ContextSym = 0x0000011B,
    // TPM_KEY_BITS, the size of the key used for encryption of saved
    // contexts
    ContextSymSize = 0x0000011C,
    // the modulus - 1 of the count for NV update of an orderly counter
    OrderlyCount = 0x0000011D,
    // the maximum value for commandSize in a command
    MaxCommandSize = 0x0000011E,
    // the maximum value for responseSize in a response
    MaxResponseSize = 0x0000011F,
    // the maximum size of a digest that can be produced by the TPM
    MaxDigest = 0x00000120,
    // the maximum size of an object context that will be returned by
    // TPM2_ContextSave
    MaxObjectContext = 0x00000121,
    // the maximum size of a session context that will be returned by
    // TPM2_ContextSave
    MaxSessionContext = 0x00000122,
    // platform-specific family (a TPM_PS value)(see Table 25)
    PSFamilyIndicator = 0x00000123,
    // the level of the platform-specific specification
    PSLevel = 0x00000124,
    // a platform specific value
    PSRevision = 0x00000125,
    // the platform-specific TPM specification day of year using TCG
    // calendar
    PSDayOfYear = 0x00000126,
    // the platform-specific TPM specification year using the CE
    PSYear = 0x00000127,
    // the number of split signing operations supported by the TPM
    SplitMax = 0x00000128,
    // total number of commands implemented in the TPM
    TotalCommands = 0x00000129,
    // number of commands from the TPM library that are implemented
    LibraryCommands = 0x0000012A,
    // number of vendor commands that are implemented
    VendorCommands = 0x0000012B,
    // the maximum data size in one NV write, NV read, NV extend, or NV
    // certify command
    NVBufferMax = 0x0000012C,
    // a TPMA_MODES value, indicating that the TPM is designed for these
    // modes.
    Modes = 0x0000012D,
    // the maximum size of a TPMS_CAPABILITY_DATA structure returned in
    // TPM2_GetCapability().
    MaxCapBuffer = 0x0000012E,
    // TPMA_PERMANENT
    Permanent = 0x00000200,
    // TPMA_STARTUP_CLEAR
    StartupClear = 0x00000201,
    // the number of NV Indexes currently defined
    HRNVIndex = 0x00000202,
    // the number of authorization sessions currently loaded into TPM RAM
    HRLoaded = 0x00000203,
    // the number of additional authorization sessions, of any type, that
    // could be loaded into TPM RAM
    HRLoadedAvail = 0x00000204,
    // the number of active authorization sessions currently being tracked
    // by the TPM
    HRActive = 0x00000205,
    // the number of additional authorization sessions, of any type, that
    // could be created
    HRActiveAvail = 0x00000206,
    // estimate of the number of additional transient objects that could be
    // loaded into TPM RAM
    HRTransientAvail = 0x00000207,
    // the number of persistent objects currently loaded into TPM NV memory
    HRPersistent = 0x00000208,
    // the number of additional persistent objects that could be loaded into
    // NV memory
    HRPersistentAvail = 0x00000209,
    // the number of defined NV Indexes that have NV the TPM_NT_COUNTER
    // attribute
    NVCounters = 0x0000020A,
    // the number of additional NV Indexes that can be defined with their
    // TPM_NT of TPM_NV_COUNTER and the TPMA_NV_ORDERLY attribute SET
    NVCountersAvail = 0x0000020B,
    // code that limits the algorithms that may be used with the TPM
    AlgorithmSet = 0x0000020C,
    // the number of loaded ECC curves
    LoadedCurves = 0x0000020D,
    // the current value of the lockout counter (failedTries)
    LockoutCounter = 0x0000020E,
    // the number of authorization failures before DA lockout is invoked
    MaxAuthFail = 0x0000020F,
    // the number of seconds before the value reported by
    // TPM_PT_LOCKOUT_COUNTER is decremented
    LockoutInterval = 0x00000210,
    // the number of seconds after a lockoutAuth failure before use of
    // lockoutAuth may be attempted again
    LockoutRecovery = 0x00000211,
    // number of milliseconds before the TPM will accept another command
    // that will modify NV
    NVWriteRecovery = 0x00000212,
    // the high-order 32 bits of the command audit counter
    AuditCounter0 = 0x00000213,
    // the low-order 32 bits of the command audit counter
    AuditCounter1 = 0x00000214,
}
