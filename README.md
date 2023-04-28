# `to_phantom`

[![Crates.io](https://img.shields.io/crates/v/to_phantom)](https://crates.io/crates/to_phantom)
[![Docs](https://img.shields.io/docsrs/to_phantom)](https://docs.rs/to_phantom/latest/to_phantom/)
[![License](https://img.shields.io/crates/l/to_phantom)](https://github.com/MrGVSV/to_phantom/blob/main/License.md)

Easily convert [`Generics`](https://docs.rs/syn/latest/syn/struct.Generics.html)
to [`PhantomData`](https://doc.rust-lang.org/core/marker/struct.PhantomData.html) in your proc macros.

This is useful for when creating custom types in a proc macro that use the generics from some other type.
The `PhantomData` allows those generics to exist on the type without needing dedicated fields using them.

```rust
use to_phantom::ToPhantom;

fn create_helper(input: DeriveInput) -> TokenStream {
    let generics = input.generics();
    let phantom = generics.to_phantom();

    quote! {
        pub struct MyHelperStruct #generics {
            phantom: #phantom,
        }
    }
}
```