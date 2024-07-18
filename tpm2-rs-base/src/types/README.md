# tpm2-rs-base::types

The types under this module are separated based on the common prefix as
specified by the TPM2 specification. There is no strong reason to separate these
by name, other than ease of organization.

These prefixes are defined below, the explanations pulled directly from the TPM2
specification (note this naming convention just suggests what kind of structure
the type is, nothing more or less; it helps us know what we are dealing with).

## TPM

A constant or an enumerated type.

## TPM2B

A structure that is a sized buffer where the size of the buffer is contained in
a 16-bit, unsigned value. The first parameter is the size in octets of the
second parameter. The second parameter may be any type.

## TPMA

A structure where each of the fields defines an attribute and each field is
usually a single bit. All the attributes in an attribute structure are packed
with the overall size of the structure indicated in the heading of the attribute
description (UINT8, UINT16, or UINT32).

## TPMI

An interface type. The value is specified for purposes of dynamic type checking
when unmarshaled.

## TPML

A list length followed by the indicated number of entries of the indicated type.
This is an array with a length field.

## TPMS

A structure that is not a size buffer or a tagged buffer or a list.

## TPMT

A structure with the first parameter being a structure tag, indicating the type
of the structure that follows. A structure tag may be either a `TPM_ST_` or
`TPM_ALG_` depending on context.

## TPMU

A union of structures, lists, or unions. If a union exists, there will normally
be a companion `TPMT_` that is the expression of the union in a tagged
structure, where the tag is the selector indicating which member of the union is
present.
