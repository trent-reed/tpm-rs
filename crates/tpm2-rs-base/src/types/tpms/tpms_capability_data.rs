use super::*;

#[repr(C, u32)]
#[derive(Clone, Copy, PartialEq, Debug, Marshalable)]
pub enum TpmsCapabilityData {
    Algorithms(TpmlAlgProperty) = TPM2Cap::Algs.0,
    Handles(TpmlHandle) = TPM2Cap::Handles.0,
    Command(TpmlCca) = TPM2Cap::Commands.0,
    PpCommands(TpmlCc) = TPM2Cap::PPCommands.0,
    AuditCommands(TpmlCc) = TPM2Cap::AuditCommands.0,
    AssignedPcr(TpmlPcrSelection) = TPM2Cap::PCRs.0,
    TpmProperties(TpmlTaggedTpmProperty) = TPM2Cap::TPMProperties.0,
    PcrProperties(TpmlTaggedPcrProperty) = TPM2Cap::PCRProperties.0,
    EccCurves(TpmlEccCurve) = TPM2Cap::ECCCurves.0,
    AuthPolicies(TpmlTaggedPolicy) = TPM2Cap::AuthPolicies.0,
}
