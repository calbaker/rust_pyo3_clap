use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens}; // ToTokens is implicitly used as a trait
use syn::{spanned::Spanned, Meta};

#[derive(Debug, Default, Clone)]
struct FieldOptions {
    /// if true, getters are not generated for a field
    pub skip_get: bool,
    /// if true, setters are not generated for a field
    pub skip_set: bool,
}

macro_rules! impl_getters_and_setters {
    ($impl_block: ident, $field: ident, $type: ident) => {
        let get_name: TokenStream2 = format!("get_{}", $field).parse().unwrap();
        let set_name: TokenStream2 = format!("set_{}", $field).parse().unwrap();
        let set_err: TokenStream2 = format!("set_{}_err", $field).parse().unwrap();
        let setter_rename: TokenStream2 = format!("__{}", $field).parse().unwrap();

        $impl_block.extend::<TokenStream2>(quote! {
            #[getter]
            pub fn #get_name(&self) -> PyResult<#$type> {
                Ok(self.#$field.clone())
            }
        });

        // directly setting value raises error to prevent nested struct issues
        $impl_block.extend::<TokenStream2>(quote! {
            #[setter]
            pub fn #set_name(&mut self, new_val: #$type) -> PyResult<()> {
                Err(PyAttributeError::new_err("Directing setting is disabled to prevent nested struct errors."))
            }
        });

        // setting is enabled indirectly becaues pyo3 does not support setting values in nested structs
        $impl_block.extend::<TokenStream2>(quote! {
            #[setter(#setter_rename)]
            pub fn #set_err(&mut self, new_val: #$type) -> PyResult<()> {
                self.#$field = new_val;
                Ok(())
            }
        });

    };
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn pyo3_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::ItemStruct);
    let ident = &ast.ident;
    // println!("{}", String::from("*").repeat(30));
    // println!("struct: {}", ast.ident.to_string());

    let mut impl_block = TokenStream2::default();
    // add in custom, struct-specific methods provided as inputs
    impl_block.extend::<TokenStream2>(attr.into());

    if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = &mut ast.fields {
        for field in named.iter_mut() {
            let ident = field.ident.as_ref().unwrap();
            let ftype = field.ty.clone();

            // if attr.tokens.to_string().contains("skip_get"){
            // for (i, idx_del) in idxs_del.into_iter().enumerate() {
            //     attr_vec.remove(*idx_del - i);
            // }

            // this is my quick and dirty attempt at emulating:
            // https://github.com/PyO3/pyo3/blob/48690525e19b87818b59f99be83f1e0eb203c7d4/pyo3-macros-backend/src/pyclass.rs#L220

            let mut opts = FieldOptions::default();
            let keep: Vec<bool> = field
            .attrs
            .iter()
            .map(|x| match x.path.segments[0].ident.to_string().as_str() { // todo: check length of segments for robustness
                "api" => {
                    let meta = x.parse_meta().unwrap();
                    if let Meta::List(list) = meta {
                        for nested in list.nested {
                            if let syn::NestedMeta::Meta(opt) = nested {
                                // println!("opt_path{:?}", opt.path().segments[0].ident.to_string().as_str());;
                                let opt_name = opt.path().segments[0].ident.to_string();
                                match  opt_name.as_str() {
                                    "skip_get" => opts.skip_get = true,
                                    "skip_set" => opts.skip_set = true,
                                    // todo: figure out how to use span to have rust-analyzer highlight the exact code
                                    // where this gets messed up
                                    _ => {
                                        abort!(
                                            x.span(),
                                            format!(
                                                "Invalid api option: {}.\nValid options are: `skip_get` and `skip_set`.",
                                                opt_name
                                            )
                                        )
                                    }
                                }
                            }
                        }
                    }
                    false
                }
                _ => true,
            })
            .collect();
            // println!("options {:?}", opts);
            let mut iter = keep.iter();
            // this drops attrs with api, removing the field attribute from the struct def
            field.attrs.retain(|_| *iter.next().unwrap());

            impl_getters_and_setters!(impl_block, ident, ftype);
        }
    } else {
        panic!("Invalid use of macro.  Works on structs with named fields only.")
    };

    impl_block.extend::<TokenStream2>(quote! {
        #[classmethod]
        #[pyo3(name = "default")]
        pub fn default_py(_cls: &PyType) -> PyResult<Self> {
            Ok(Self::default())
        }
    });

    impl_block.extend::<TokenStream2>(quote! {
        #[pyo3(name = "clone")]
        pub fn clone_py(&self) -> Self {
            self.clone()
        }
    });

    let impl_block = quote! {
        #[pymethods]
        impl #ident {
            #impl_block
        }
    };

    let mut final_output = TokenStream2::default();
    // add pyclass attribute
    final_output.extend::<TokenStream2>(quote! {
        #[pyclass]
    });
    let mut output: TokenStream2 = ast.to_token_stream();
    output.extend(impl_block);
    // println!("{}", output.to_string());
    final_output.extend::<TokenStream2>(output);
    final_output.into()
}
