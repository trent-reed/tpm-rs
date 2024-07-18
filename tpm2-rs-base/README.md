# tpm2-rs-base

The core foundational crate for the rest of TPM-RS. If anything is so common
that it is needed across every project, it will be implemented in this crate.

## Commands

For clarity of intent, commands (and the types directly related to a command,
such as a response object) are defined in the `commands` module.

We don't re-export this module so that there is a nice separation between the
special command types and the specification-defined types.

## Constants

Any values which are just constants will be defined here. Don't confuse this
with enum variants (which themselves define constants but are otherwise a
separate type, so they are in the `types` module).

This module is re-exported in the root of the crate.

## Types

This crate provides a number of types based on the TPM2 specification. The names
of the types have been preserved for ease of mapping type names from the spec to
the types in the module.

Each type is defined in a separate file to encourage proper separation of
definitions, and allow restriction for access of private functionality (should
any custom functionality need added).

The types are further organized into submodules based on the naming convention
provided in the spec. Here is a quick synopsis of the name prefixes (provided
verbatim from the TPM2 specification).

This organization is solely for ease of development and enforcement of proper
visibility constraints in rust. External clients are not expected to know about
this module explicitly.

This module is re-exported in the root of the crate.
