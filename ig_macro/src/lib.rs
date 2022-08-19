extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, parse::Parser};

#[proc_macro_attribute]
#[allow(unused_variables)]
pub fn agent_fields(args:TokenStream, input:TokenStream)->TokenStream{
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data{
        syn::Data::Struct(ref mut struct_data) =>{
            match &mut struct_data.fields{
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote!{
                        pub attr:AgentAttr
                    }).unwrap());
                }
                _=>{()}
            }
            return quote!{
                #ast
            }.into();
        }
        _=>panic!("agent_field must be used with struct"),
    }
}

#[proc_macro_attribute]
#[allow(unused_variables)]
pub fn agent_methods(args:TokenStream, input:TokenStream)->TokenStream{
    let mut str : String  = "".to_owned();
    for tt in input.into_iter(){
        match tt {
            TokenTree::Group(tgroup) =>{
                str.push_str("{\n");
                for tt2 in tgroup.stream().into_iter(){
                    match tt2{
                        TokenTree::Ident(tident)=>{
                            str.push_str(format!("{} ",tident).as_str());
                        }
                        TokenTree::Group(tgroup2)=>{
                            str.push_str(format!("{}",tgroup2).as_str());
                        }
                        _=>{
                            str.push_str(format!("{}",tt2).as_str());
                        }
                    }
                }
                str.push_str("#[allow(unused_variables)] fn attr(&mut self)->&mut AgentAttr{ &mut self.attr }\n");
                str.push_str("#[allow(unused_variables)] fn read_attr(&self)->&AgentAttr{ &self.attr }\n");
                str.push_str("}\n");
            },
            _=>{
                str.push_str(format!("{} ",tt).as_str());
            }
        }
    }
    //panic!("{}", str);
    //input
    str.parse().unwrap()

}
