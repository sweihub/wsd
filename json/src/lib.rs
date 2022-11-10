extern crate proc_macro;
use std::str::FromStr;
use proc_macro::TokenStream;

mod json;
use json::*;
use syn::parse_macro_input;

/*/
fn check(expr: &Expr) {
    //println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");    
    match expr {
        Expr::Assign(_) => { println!("Assign"); },
        Expr::AssignOp(_) => { println!("AssignOp"); },
        Expr::Async(_) => { println!("Async"); },
        Expr::Await(_) => { println!("Await"); },
        Expr::Binary(_) => { println!("Block"); },
        Expr::Block(_) => { println!("Assign"); },
        Expr::Box(_) => { println!("Box"); },
        Expr::Break(_) => { println!("Break"); },
        Expr::Call(_) => { println!("Call"); },
        Expr::Cast(_) => { println!("Cast"); },
        Expr::Closure(_) => { println!("Closure"); },
        Expr::Continue(_) => { println!("Continue"); },
        Expr::ForLoop(_) => { println!("ForLoop"); },
        Expr::Group(_) => { println!("Group"); },
        Expr::If(_) => { println!("If"); },
        Expr::Index(_) => { println!("Index"); },
        Expr::Let(_) => { println!("Let"); },
        // literal 
        Expr::Lit(_) => { println!("Lit"); },
        Expr::Loop(_) => { println!("Loop"); },
        Expr::Macro(_) => { println!("Macro"); },
        Expr::Match(_) => { println!("Match"); },
        Expr::Paren(_) => { println!("Paren"); },
        Expr::Path(_) => { println!("Path"); },
        Expr::Range(_) => { println!("Range"); },
        Expr::Reference(_) => { println!("Reference"); },
        Expr::Repeat(_) => { println!("Repeat"); },
        Expr::Return(_) => { println!("Return"); },
        Expr::Struct(_) => { println!("Struct"); },
        Expr::Try(_) =>  { println!("Try"); },
        Expr::TryBlock(_) => { println!("TryBlock"); },
        Expr::Tuple(_) => { println!("Tuple"); },
        Expr::Type(_) => { println!("Type"); },
        Expr::Unary(_) => { println!("Unary"); },
        Expr::Unsafe(_) => { println!("Unsafe"); },
        Expr::Verbatim(_) => { println!("Verbatim"); },
        Expr::While(_) => { println!("While"); },
        Expr::Yield(_) => { println!("Yield"); },
        _ => todo!(),
    }
    println!("=================================");    
    println!("{:#?}", expr);
}
*/

/*
fn wrap_struct_name( struct_name: &str, input: TokenStream ) -> TokenStream {
    let mut ts = TokenStream::from( Ident::new( struct_name, Span::call_site() ).into_token_stream() );
    ts.extend( Some( TokenTree::Group( Group::new( Delimiter::Brace, input ))));
    ts
}
*/

/* 
fn traverse(input: TokenStream)  -> TokenStream {

    //let a = parse_macro_input!(input as Struct);   

    let id =  proc_macro::Ident::new("structx_name_x", proc_macro::Span::call_site());
    let mut ts = TokenStream::from(TokenTree::Ident(id));
    let group = Group::new(Delimiter::Brace, input);
    let g = TokenStream::from(TokenTree::Group(group));
    ts.extend(g);

    /*/
    let output = input.into_iter().map(|x| {
        match x {
            TokenTree::Group(i) => { TokenTree::Group(i)},
            TokenTree::Ident(i) => { TokenTree::Ident(i)},
            TokenTree::Punct(i) => { TokenTree::Punct(i)},
            TokenTree::Literal(i) => { TokenTree::Literal(i)}
        }
    }).collect();
    */

    println!("XXXXXXXXXXXXXXXXXXXXXX\n{:#?}", ts);

    return ts;
}
*/

#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {

    let root = parse_macro_input!(input as Json); 
    let prototypes = root.get_pototypes();
    let code = root.get_code();
    println!("XXXXXXXX\n{}", prototypes);
    println!("XXXXXXXX\n{}", code);  

    let block = format!("{{ {}\n{} }}", prototypes, code);

    return TokenStream::from_str(block.as_str()).unwrap();    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}