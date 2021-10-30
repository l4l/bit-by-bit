extern crate proc_macro;

use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse, Token};

#[derive(PartialEq, Eq, Clone, Copy)]
struct TypeInfo {
    bittness: usize,
    is_signed: bool,
}

impl TypeInfo {
    fn from_type_path(path: &syn::TypePath) -> Self {
        assert!(path.qself.is_none());
        let ty_str = path
            .path
            .get_ident()
            .unwrap_or_else(|| {
                let path = &path.path;
                panic!("Unsupported type `{}`", quote!(#path))
            })
            .to_string();
        match ty_str.as_str() {
            "u8" => TypeInfo {
                bittness: 8,
                is_signed: false,
            },
            "i8" => TypeInfo {
                bittness: 8,
                is_signed: true,
            },
            "u16" => TypeInfo {
                bittness: 16,
                is_signed: false,
            },
            "i16" => TypeInfo {
                bittness: 16,
                is_signed: true,
            },
            "u32" => TypeInfo {
                bittness: 32,
                is_signed: false,
            },
            "i32" => TypeInfo {
                bittness: 32,
                is_signed: true,
            },
            "u64" => TypeInfo {
                bittness: 64,
                is_signed: false,
            },
            "i64" => TypeInfo {
                bittness: 64,
                is_signed: true,
            },
            "u128" => TypeInfo {
                bittness: 128,
                is_signed: false,
            },
            "i128" => TypeInfo {
                bittness: 128,
                is_signed: true,
            },
            s => panic!("Unsupported type `{}`", s),
        }
    }

    fn from_type(ty: &syn::Type) -> Self {
        match ty {
            syn::Type::Path(ref path) => Self::from_type_path(path),
            syn::Type::Group(group) => Self::from_type(&group.elem),
            ty => panic!(
                "Only primitive types are supported, but provided `{}`",
                quote!(#ty)
            ),
        }
    }
}

impl quote::ToTokens for TypeInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let s = format!(
            "{}{}",
            if self.is_signed { 'i' } else { 'u' },
            self.bittness
        );
        let stream = proc_macro::TokenStream::from_str(&s).unwrap();
        let ty: syn::Type = parse(stream).unwrap();
        tokens.extend(quote! { #ty })
    }
}

struct PrevField {
    ty_info: TypeInfo,
    bits_left: usize,
    common_field_name: String,
}

struct FieldInfo {
    field_name: String,
    common_field_name: String,
    ty_info: TypeInfo,
    start_bit: usize,
    end_bit: usize,
}

#[proc_macro_attribute]
pub fn bit_by_bit(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        attrs,
        vis,
        ident,
        generics,
        fields,
        ..
    } = parse(input).expect("Expected a struct item");

    let fields = match fields {
        syn::Fields::Named(fields) => fields.named,
        _ => panic!("Only structs with named fields are supported"),
    };

    let mut replaced_fileds = Vec::<FieldInfo>::new();
    let mut new_fields = 0;
    let mut prev_field = None::<PrevField>;

    let fields = fields
        .into_iter()
        .filter_map(|mut f| {
            if let Some(idx) = f
                .attrs
                .iter()
                .position(|attr| attr.path.get_ident().unwrap().to_string().as_str() == "bit")
            {
                let ty_info = TypeInfo::from_type(&f.ty);
                let bits = f
                    .attrs
                    .remove(idx)
                    .parse_args::<syn::LitInt>()
                    .unwrap()
                    .base10_parse::<usize>()
                    .unwrap();
                assert!(
                    bits <= ty_info.bittness,
                    "bitness overflow, the type support up to {} bit, but {} provided",
                    ty_info.bittness,
                    bits
                );

                let field_name = f
                    .ident
                    .as_ref()
                    .expect("fields are always named")
                    .to_string();

                if let Some(mut prev) = prev_field
                    .take()
                    .filter(|prev| prev.bits_left >= bits && prev.ty_info == ty_info)
                {
                    let start_bit = ty_info.bittness - prev.bits_left;
                    prev.bits_left -= bits;
                    let end_bit = ty_info.bittness - prev.bits_left;
                    replaced_fileds.push(FieldInfo {
                        field_name,
                        common_field_name: prev.common_field_name.clone(),
                        ty_info,
                        start_bit,
                        end_bit,
                    });
                    prev_field = Some(prev);
                    return None;
                } else {
                    let common_field_name = format!("__base_field_{}", new_fields);
                    replaced_fileds.push(FieldInfo {
                        field_name,
                        common_field_name: common_field_name.clone(),
                        ty_info,
                        start_bit: 0,
                        end_bit: bits,
                    });

                    let ident = f.ident.as_mut().expect("checked earlier");
                    let span = ident.span();
                    *ident = syn::Ident::new(&common_field_name, span);

                    prev_field = Some(PrevField {
                        ty_info,
                        bits_left: ty_info.bittness - bits,
                        common_field_name,
                    });
                    new_fields += 1;
                }
            }

            Some(f)
        })
        .collect::<Punctuated<_, Token![,]>>();

    let fns = replaced_fileds
        .iter()
        .map(|info| {
            let FieldInfo {
                field_name,
                common_field_name,
                ty_info,
                start_bit,
                end_bit,
            } = info;
            let field_ident = quote::format_ident!("{}", field_name);
            let set_field_ident = quote::format_ident!("set_{}", field_name);
            let cfn = quote::format_ident!("{}", common_field_name);
            let mut mask = 0u128;
            for _ in 0..*end_bit {
                mask <<= 1;
                mask |= 1;
            }
            let mask = syn::LitInt::new(&mask.to_string(), proc_macro2::Span::call_site());

            quote! {
                fn #field_ident (&self) -> #ty_info {
                    (self.#cfn >> #start_bit) & #mask
                }

                fn #set_field_ident (&mut self, val: #ty_info) {
                    self.#cfn ^= (self.#cfn >> #start_bit) & #mask;
                    self.#cfn |= (val & #mask) << #start_bit;
                }
            }
        })
        .collect::<Vec<_>>();

    let generic_params = &generics.params;
    let generic_param_names = generics
        .params
        .iter()
        .map(|param| {
            use syn::GenericParam;
            match &param {
                GenericParam::Type(ty) => {
                    let ident = &ty.ident;
                    quote! { #ident }
                }
                GenericParam::Lifetime(lf) => {
                    let lifetime = &lf.lifetime;
                    quote! { #lifetime }
                }
                GenericParam::Const(c) => {
                    let ident = &c.ident;
                    quote! { #ident }
                }
            }
        })
        .collect::<Punctuated<_, Token![,]>>();
    let generic_where = &generics.where_clause;

    let item = quote! {
        #(#attrs)*
        #vis struct #ident #generics
        #generic_where
        {
            #fields
        }

        impl < #generic_params > #ident < #generic_param_names >
        #generic_where
        {
            #(#fns)*
        }
    };

    item.into()
}
