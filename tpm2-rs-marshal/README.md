# tpm2-rs-marshal

An implementation of the derive macro for marshalable types.

## Marshalable

Marshalable is both the trait and the derive macro for the type. The trait
allows us to marshal and unmarshal types in Rust to bytes that a TPM can
understand.

### Structs

All manner of structs support the Marshalable derive trait.

```rs
#[derive(Marshalable)]
struct Struct1;

#[derive(Marshalable)]
struct Struct2(OtherMarshalableType1, OtherMarshalableType2);

#[derive(Marshalable)]
struct Struct3 {
   named1: OtherMarshalableType1,
   named2: OtherMarshalableType2,
   named3: OtherMarshalableType3,
}
```

The code generated will marshal the structure by it's fields (named or unnamed)
in the order that they are listed. If there are no fields the marshalable type
takes no bytes to marshal or unmarshal.

Structs have another feature that allows for out-of-line definitions for how
much data is actually filled in for an array.

```rs
#[derive(Marshalable)]
struct Struct {
    value_count: u16,
    #[marshal(length=value_count)]
    values: [OtherMarshalableType; 10],
}
```

This will fail if `value_count` is greater than the max length defined by the
array provided in `values`. And when marshalling it will only marshal up-to the
total number of values as suggested by `value_count`.

### Enums

Enums also can be marshaled:

```rs
#[repr(u8)]
#[derive(Marshalable)]
enum Enum {
    Variant1 = 1,
    Variant2(OtherMarshalableType1, OtherMarshalableType2) = 2,
    Variant3{
        named1: OtherMarshalableType1,
        named2: OtherMarshalableType2,
        named3: OtherMarshalableType3,
    } = 3,
}
```

Enums are always marshaled in the format of the discriminant first, followed by
the data required for the variant defined by that discriminant. There are a few
requirements on a marshalable enum though:

1.  The enum must be `#[repr(T)]` where `T` is `u8`, `u16`, `u32`, or `u64`.
2.  The enum must have at least one defined variant (since you cannot construct
    an enum with no values).
3.  All variants must have an explicitly defined descriminant (this can be done
    by adding `= N` where `N` is a number within the range of the `repr` type to
    the end of the variant definition).

Note that this type does not support the `#[marshal(length=...)]` attribute. Not
for any reason other than it is not implemented (we should probably support this
though, there's no reason it should not be supported).
