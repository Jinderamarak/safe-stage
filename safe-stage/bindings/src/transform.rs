use proc_macro2::{Ident, Span, TokenTree};
use quote::ToTokens;
use std::collections::VecDeque;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_file, token, Attribute, Field, Fields, FnArg, Generics, ImplItem, ImplItemFn, Item,
    ItemEnum, ItemFn, ItemImpl, Meta, Pat, PatIdent, PatType, Receiver, ReturnType, Type,
    TypeReference, Variant, Visibility,
};

#[derive(Debug, Clone, PartialEq)]
struct TaggedUnionRepr {
    data_repr: Ident,
    tag_repr: Ident,
}

impl TaggedUnionRepr {
    fn data_attr(&self) -> Attribute {
        let t = &self.data_repr;
        syn::parse_quote!(#[repr(#t)])
    }

    fn tag_attr(&self) -> Attribute {
        let t = &self.tag_repr;
        syn::parse_quote!(#[repr(#t)])
    }
}

/// Transforms Rust source code into a form suitable for C# bindings generation.
pub fn transform_rust_source_code(source_code: &str) -> String {
    let mut syntax_tree = parse_file(source_code).expect("Failed to parse the Rust file");

    // Processed items
    let mut transformed_items = Vec::new();

    // Queue of items to process
    let mut queue = syntax_tree.items.into_iter().collect::<VecDeque<Item>>();
    while let Some(item) = queue.pop_front() {
        match item {
            /*
                Flattening of public modules.
                Example:
                ```
                pub mod foo {
                    pub mod bar {
                        pub fn baz() {}
                    }
                    mod priv {
                        pub fn bazinga() {}
                    }
                }
                ```
                will be iteratively transformed down to to
                ```
                pub fn baz() {}
                mod priv {
                    pub fn bazinga() {}
                }
                ```
            */
            Item::Mod(module) => {
                if is_public(&module.vis) {
                    if let Some((_, items)) = module.content {
                        queue.extend(items);
                    }
                } else {
                    transformed_items.push(Item::Mod(module));
                }
            }
            /*
                Transforming of enums with values to explicit tagged unions.
                The Enum must be decorated with `#[repr(C, i32)]`, where the first argument
                should be `C` and second any `Int`.
                Example:
                ```
                #[repr(C, i32)]
                pub enum Foo {
                    Bar(i32),
                    Baz(f64),
                }
                ```
                will be transformed to
                ```
                #[repr(C)]
                pub struct Foo_Bar(i32);
                #[repr(C)]
                pub struct Foo_Baz(f64);
                #[repr(C)]
                pub union Foo_Data {
                    Bar: Foo_Bar,
                    Baz: Foo_Baz,
                }
                #[repr(i32)]
                pub enum Foo_Tag {
                    Bar,
                    Baz,
                }
                #[repr(C)]
                pub struct Foo {
                    tag: Foo_Tag,
                    data: Foo_Data,
                }
                ```
                Currently transforms only public enums without generics.
            */
            Item::Enum(item_enum) => {
                if is_public(&item_enum.vis) && !has_generics(&item_enum.generics) {
                    let first_repr = item_enum.attrs.iter().filter_map(get_repr).next();
                    if let Some(repr) = first_repr {
                        transformed_items.extend(transform_enum(item_enum, repr));
                    } else {
                        if is_tagged_union(&item_enum) && has_repr(&item_enum) {
                            const START_YELLOW_BOLD: &str = "\x1b[33;49;1m";
                            const RESET: &str = "\x1b[0m";
                            println!("{START_YELLOW_BOLD}WARNING:{RESET} Tagged union has `repr` attribute but in wrong format. Did you mean `#[repr(C, IntType)]`? Type name: {}", item_enum.ident);
                        }
                        //  missing correct `repr`, leave as-is
                        transformed_items.push(Item::Enum(item_enum))
                    }
                } else {
                    //  public or generic, leave as-is
                    transformed_items.push(Item::Enum(item_enum))
                }
            }
            /*
               Extracting public extern functions into standalone functions.
               Example:
               ```
               impl Foo {
                   pub extern "C" fn bar(&self) {}
                   fn baz(&self) -> Self {}
               }
               ```
               will be transformed to
               ```
               pub extern "C" fn bar(myself: &Foo) -> Foo {}
               impl Foo {
                   fn baz(&self) {}
               }
               ```
            */
            Item::Impl(implementation) => {
                transformed_items.extend(transform_impl(implementation));
            }
            other => {
                // Retain other items as-is
                transformed_items.push(other);
            }
        }
    }

    syntax_tree.items = transformed_items;
    syntax_tree.into_token_stream().to_string()
}

/// Check if the visibility is public.
fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

/// Check if the function is an ABI function, i.e. has `extern` keyword.
fn is_abi_fn(fun: &ImplItemFn) -> bool {
    fun.sig.abi.is_some()
}

/// Check if the generics are actual generics.
fn has_generics(generics: &Generics) -> bool {
    generics.gt_token.is_some() && generics.lt_token.is_some()
}

/// Check if the enum has `repr` attribute.
fn has_repr(item: &ItemEnum) -> bool {
    item.attrs.iter().any(|attr| {
        if let Meta::List(list) = &attr.meta {
            list.path.is_ident("repr")
        } else {
            false
        }
    })
}

/// Check if any of the variants has fields.
fn is_tagged_union(item: &ItemEnum) -> bool {
    for variant in &item.variants {
        match &variant.fields {
            Fields::Named(_) | Fields::Unnamed(_) => return true,
            Fields::Unit => continue,
        }
    }

    false
}

/// Extracts public extern functions into standalone functions.
fn transform_impl(mut implementation: ItemImpl) -> Vec<Item> {
    if implementation.trait_.is_some() {
        return vec![Item::Impl(implementation)];
    }

    let mut impled = Vec::new();
    let mut standalone = Vec::new();
    for item in implementation.items {
        match item {
            ImplItem::Fn(fun) => {
                if is_public(&fun.vis) && is_abi_fn(&fun) {
                    standalone.push(transform_fn(fun, implementation.self_ty.clone()));
                } else {
                    impled.push(ImplItem::Fn(fun));
                }
            }
            other => {
                impled.push(other);
            }
        }
    }

    implementation.items = impled;
    standalone.push(Item::Impl(implementation));
    standalone
}

/// Transforms function from `Impl` block into standalone function
/// by expanding references to `self` and `Self`.
fn transform_fn(fun: ImplItemFn, parent: Box<Type>) -> Item {
    let mut fun = ItemFn {
        attrs: fun.attrs,
        vis: fun.vis,
        sig: fun.sig,
        block: Box::new(fun.block),
    };

    if let ReturnType::Type(t, ret) = fun.sig.output.clone() {
        if let Type::Path(path) = &*ret {
            let probably_self = path
                .path
                .segments
                .first()
                .map(|s| s.ident == "Self")
                .unwrap_or(false);
            if path.qself.is_some() || probably_self {
                fun.sig.output = ReturnType::Type(t, parent.clone());
            }
        }
    }

    let first_arg = fun.sig.inputs.first();
    if let Some(FnArg::Receiver(rec)) = first_arg {
        fun.sig.inputs[0] = FnArg::Typed(transform_self(rec, parent.clone()));
    }

    Item::Fn(fun)
}

/// Transforms function argument `self` into proper argument `myself` of type `parent`.
fn transform_self(rec: &Receiver, parent: Box<Type>) -> PatType {
    let attrs = rec.attrs.clone();
    let ident = Ident::new("myself", Span::call_site());
    let colon_token = token::Colon::default();

    let is_ref = rec.reference.is_some();
    let is_mut = rec.mutability.is_some();

    let mutability = if is_mut && !is_ref {
        Some(token::Mut::default())
    } else {
        None
    };

    let ty = if is_ref {
        let lifetime = rec.reference.as_ref().and_then(|r| r.1.clone());
        Box::new(Type::Reference(TypeReference {
            and_token: token::And::default(),
            lifetime,
            mutability: rec.mutability,
            elem: parent,
        }))
    } else {
        parent
    };

    PatType {
        attrs,
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability,
            ident,
            subpat: None,
        })),
        colon_token,
        ty,
    }
}

/// Tried parsing attribute `#[repr(A, B)]` into `TaggedUnionRepr`.
fn get_repr(attr: &Attribute) -> Option<TaggedUnionRepr> {
    match &attr.meta {
        Meta::List(list) => {
            if !list.path.is_ident("repr") {
                return None;
            }

            let types = list
                .tokens
                .clone()
                .into_iter()
                .filter_map(|t| {
                    if let TokenTree::Ident(ident) = t {
                        Some(ident)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            match &types[..] {
                [data, tag] => Some(TaggedUnionRepr {
                    data_repr: data.clone(),
                    tag_repr: tag.clone(),
                }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Transforms tagged-union-like enum into explicit `_Tag` enum and `_Data` union.
fn transform_enum(item_enum: ItemEnum, enum_repr: TaggedUnionRepr) -> Vec<Item> {
    let mut output = Vec::new();

    let tag_attr = enum_repr.tag_attr();
    let data_attr = enum_repr.data_attr();

    let name = item_enum.ident;
    let name_tag = Ident::new(&format!("{}_Tag", name), name.span());
    let name_union = Ident::new(&format!("{}_Data", name), name.span());

    let mut variants: Punctuated<Variant, Comma> = Punctuated::new();
    let mut data: Punctuated<Field, Comma> = Punctuated::new();

    for variant in item_enum.variants.clone() {
        let data_name = Ident::new(&format!("{}_{}", name, variant.ident), name.span());
        match variant.fields.clone() {
            Fields::Named(named) => {
                let fields = named.named;
                output.push(syn::parse_quote!(
                    #data_attr
                    pub struct #data_name {
                        #fields
                    }
                ));
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed.unnamed;
                output.push(syn::parse_quote!(
                    #data_attr
                    pub struct #data_name(#fields);
                ));
            }
            Fields::Unit => {
                output.push(syn::parse_quote!(
                    #data_attr
                    pub struct #data_name;
                ));
            }
        }

        let field_name = variant.ident.clone();
        data.push(syn::parse_quote!(
            #field_name: #data_name
        ));

        variants.push(syn::parse_quote!(
            #field_name
        ));
    }

    output.push(syn::parse_quote!(
        #tag_attr
        pub enum #name_tag {
            #variants
        }
    ));

    output.push(syn::parse_quote!(
        #data_attr
        pub union #name_union {
            #data
        }
    ));

    output.push(syn::parse_quote!(
        #data_attr
        pub struct #name {
            tag: #name_tag,
            data: #name_union,
        }
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_public() {
        let vis: Visibility = syn::parse_quote!(pub);
        let expected = true;
        let actual = is_public(&vis);
        assert_eq!(expected, actual);

        let vis: Visibility = syn::parse_quote!(pub(crate));
        let expected = false;
        let actual = is_public(&vis);
        assert_eq!(expected, actual);

        //  empty visibility is private
        let vis: Visibility = syn::parse_quote!();
        let expected = false;
        let actual = is_public(&vis);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_abi_fn() {
        let fun: ImplItemFn = syn::parse_quote!(
            pub extern "C" fn foo() {}
        );
        let expected = true;
        let actual = is_abi_fn(&fun);
        assert_eq!(expected, actual);

        let fun: ImplItemFn = syn::parse_quote!(
            pub extern "C" fn foo() {}
        );
        let expected = true;
        let actual = is_abi_fn(&fun);
        assert_eq!(expected, actual);

        let fun: ImplItemFn = syn::parse_quote!(
            pub fn foo() {}
        );
        let expected = false;
        let actual = is_abi_fn(&fun);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_has_generics() {
        let generics: Generics = syn::parse_quote!();
        let expected = false;
        let actual = has_generics(&generics);
        assert_eq!(expected, actual);

        let generics: Generics = syn::parse_quote!(<T>);
        let expected = true;
        let actual = has_generics(&generics);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_has_repr() {
        let item: ItemEnum = syn::parse_quote!(
            #[repr(C, i32)]
            pub enum Foo {
                Bar(i32),
                Baz(f64),
            }
        );
        let expected = true;
        let actual = has_repr(&item);
        assert_eq!(expected, actual);

        let item: ItemEnum = syn::parse_quote!(
            #[repr(C)]
            pub enum Foo {
                Bar(i32),
                Baz(f64),
            }
        );
        let expected = true;
        let actual = has_repr(&item);
        assert_eq!(expected, actual);

        let item: ItemEnum = syn::parse_quote!(
            #[buh(C, i32)]
            pub enum Foo {
                Bar(i32),
                Baz(f64),
            }
        );
        let expected = false;
        let actual = has_repr(&item);
        assert_eq!(expected, actual);

        let item: ItemEnum = syn::parse_quote!(
            pub enum Foo {
                Bar(i32),
                Baz(f64),
            }
        );
        let expected = false;
        let actual = has_repr(&item);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_tagged_union() {
        let item: ItemEnum = syn::parse_quote!(
            pub enum Foo {
                Bar(i32),
                Baz(f64),
            }
        );
        let expected = true;
        let actual = is_tagged_union(&item);
        assert_eq!(expected, actual);

        let item: ItemEnum = syn::parse_quote!(
            pub enum Foo {
                Bar { x: i32 },
                Baz { y: f64 },
            }
        );
        let expected = true;
        let actual = is_tagged_union(&item);
        assert_eq!(expected, actual);

        let item: ItemEnum = syn::parse_quote!(
            pub enum Foo {
                Bar { x: i32 },
                Baz(f64),
                Bum,
            }
        );
        let expected = true;
        let actual = is_tagged_union(&item);
        assert_eq!(expected, actual);

        let item: ItemEnum = syn::parse_quote!(
            pub enum Foo {
                Bar,
                Baz,
            }
        );
        let expected = false;
        let actual = is_tagged_union(&item);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_repr() {
        let attr: Attribute = syn::parse_quote!(#[repr(C)]);
        let expected = None;
        let actual = get_repr(&attr);
        assert_eq!(expected, actual);

        let attr: Attribute = syn::parse_quote!(#[repr(C, i32)]);
        let expected = Some(TaggedUnionRepr {
            data_repr: Ident::new("C", Span::call_site()),
            tag_repr: Ident::new("i32", Span::call_site()),
        });
        let actual = get_repr(&attr);
        assert_eq!(expected, actual);

        let attr: Attribute = syn::parse_quote!(#[repr(C, i32, u8)]);
        let expected = None;
        let actual = get_repr(&attr);
        assert_eq!(expected, actual);

        let attr: Attribute = syn::parse_quote!(#[buh(C, i32)]);
        let expected = None;
        let actual = get_repr(&attr);
        assert_eq!(expected, actual);

        let attr: Attribute = syn::parse_quote!(#[repr]);
        let expected = None;
        let actual = get_repr(&attr);
        assert_eq!(expected, actual);
    }
}
