/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

extern crate proc_macro;
#[macro_use]
extern crate quote;

mod util;

#[proc_macro_derive(EnumStr)]
pub fn self_tokenize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let expanded = util::expand_macro(&ast, "");
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnumStrCamelCase)]
pub fn self_tokenize_camel_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let expanded = util::expand_macro(&ast, "CamelCase");
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnumStrKebabCase)]
pub fn self_tokenize_kebab_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let expanded = util::expand_macro(&ast, "KebabCase");
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnumStrMixedCase)]
pub fn self_tokenize_mixed_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let expanded = util::expand_macro(&ast, "MixedCase");
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnumStrShoutySnakeCase)]
pub fn self_tokenize_shouty_snake_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let expanded = util::expand_macro(&ast, "ShoutySnakeCase");
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnumStrSnakeCase)]
pub fn self_tokenize_snake_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let expanded = util::expand_macro(&ast, "SnakeCase");
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EnumStrTitleCase)]
pub fn self_tokenize_title_case(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let expanded = util::expand_macro(&ast, "TitleCase");
    proc_macro::TokenStream::from(expanded)
}
