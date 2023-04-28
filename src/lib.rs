use syn::{Generics, parse_quote, Token, Type};
use syn::punctuated::Punctuated;

pub trait ToPhantom {
    /// Creates a [`PhantomData`] based on `Self`.
    ///
    /// The `PhantomData` will be `Send` and `Sync`.
    ///
    /// The resulting `Type` will look something like:
    ///
    /// ```
    /// # use std::marker::PhantomData;
    /// # fn demo<'a, 'b, T, U>() {
    /// # let _phantom:
    /// PhantomData<(fn(&'a (), &'b ()) -> (T, U))>
    /// # ;
    /// # }
    /// ```
    ///
    /// [`PhantomData`]: core::marker::PhantomData
    fn to_phantom(&self) -> Type;
}

impl ToPhantom for Generics {
    fn to_phantom(&self) -> Type {
        let lifetimes = self.lifetimes().map(|param| {
            &param.lifetime
        }).collect::<Vec<_>>();

        let types = self.type_params().map(|param| {
            &param.ident
        }).collect::<Punctuated<_, Token![,]>>();

        parse_quote!(::core::marker::PhantomData<fn(#(&#lifetimes ()),*) -> (#types)>)
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::{Generics, parse_quote};
    use crate::ToPhantom;

    #[test]
    fn maps_empty_generics() {
        let generics: Generics = parse_quote!(<>);
        let phantom = generics.to_phantom();
        let output = phantom.to_token_stream().to_string();
        assert_eq!(":: core :: marker :: PhantomData < fn () -> () >", output);
    }

    #[test]
    fn maps_lifetimes_generics() {
        let generics: Generics = parse_quote!(<'a, 'b>);
        let phantom = generics.to_phantom();
        let output = phantom.to_token_stream().to_string();
        assert_eq!(":: core :: marker :: PhantomData < fn (& 'a () , & 'b ()) -> () >", output);
    }

    #[test]
    fn maps_type_generics() {
        let generics: Generics = parse_quote!(<T, U>);
        let phantom = generics.to_phantom();
        let output = phantom.to_token_stream().to_string();
        assert_eq!(":: core :: marker :: PhantomData < fn () -> (T , U) >", output);
    }

    #[test]
    fn maps_const_type_generics() {
        let generics: Generics = parse_quote!(<const T: usize, const U: usize>);
        let phantom = generics.to_phantom();
        let output = phantom.to_token_stream().to_string();
        assert_eq!(":: core :: marker :: PhantomData < fn () -> () >", output);
    }

    #[test]
    fn maps_all_generics() {
        let generics: Generics = parse_quote!(<'a, 'b: 'a, T: Default, const N: usize>);
        let phantom = generics.to_phantom();
        let output = phantom.to_token_stream().to_string();
        assert_eq!(":: core :: marker :: PhantomData < fn (& 'a () , & 'b ()) -> (T) >", output);
    }
}
