use std::fmt::format;

use proc_macro::TokenTree;
use syn::{*, parse::{ParseStream, Parse}};

//------------------- JSON Syntax ------------------------------
//
// object = { pair, ...}
// pair = key : value
// key = identifier
// array  = [value, ...]
// value =  object | array | expression
// expression = string | number | identifier

pub const OBJECT: u8 = 1;
pub const ARRAY: u8 = 2;
pub const EXPRESSION: u8 = 3;

pub struct Value {
    pub t: u8,
    pub i: usize,
}

pub struct Array {
    pub items: Vec<Value>,
}

pub struct Pair {
    pub key: Ident,
    pub value: Value
}


pub struct Object {
    pub id: String,
    pub pairs: Vec<Pair>
}

pub struct Json {
    pub value: Value,
    pub id: i32,
    objects: Vec<Object>,
    arrays: Vec<Array>,
    expressions: Vec<Expr>,
}

impl Array {
    pub fn new() -> Self {
        Self {items: Vec::new() }
    }
}

impl Object {
    pub fn new() -> Self {
        Self { id: "".to_string(), pairs: Vec::new() }
    }
}

impl Json {
    pub fn new() -> Self {
        return Self {
            value: Value { t: 0, i: 0 },
            id: 0,
            objects: Vec::new(),
            arrays: Vec::new(),
            expressions: Vec::new(),
        };    
    }

    pub fn get_object(&self, v : &Value) -> &Object {        
        return &self.objects[v.i];
    }

    pub fn get_array(&self, v : &Value) -> &Array {        
        return &self.arrays[v.i];
    }
    
    pub fn get_expression(&self, v : &Value) -> &Expr {        
        return &self.expressions[v.i];
    }

    fn append_object(&mut self, v: Object)  -> Value
    {
        self.objects.push(v);
        let i = self.objects.len() - 1;
        let t = OBJECT;
        return Value {t, i};
    }

    fn append_array(&mut self, v: Array)  -> Value
    {
        self.arrays.push(v);
        let i = self.arrays.len() - 1;
        let t = ARRAY;
        return Value {t, i};
    }

    fn append_expression(&mut self, v: Expr)  -> Value
    {
        self.expressions.push(v);
        let i = self.expressions.len() - 1;
        let t = EXPRESSION;
        return Value {t, i};
    }
    
    // terminal
    fn parse_expression(&mut self, input: ParseStream) -> Result<Value> 
    { 
        println!("XXXXXXXX parse expression!");        
        let expr: Expr = input.parse::<Expr>()?;
        let value = self.append_expression(expr);
        return Ok(value);
    }
    
    fn parse_pair(&mut self, input: ParseStream) -> Result<Pair> 
    {  
        println!("XXXXXXXX parse pair!");

        // key
        let key: Ident = input.parse()?;
        // :
        input.parse::<Token![:]>()?;
        // value            
        let value = self.parse_value(&input)?;

        return Ok(Pair {key, value});
    }

    // object := { key: value, ...}
    fn parse_object(&mut self, input: ParseStream) -> Result<Value> 
    {
        let inner;
        let mut content;
        let mut types: Vec<String> = Vec::new();
        let mut fields: Vec<String>;
        let mut t = 0;
        
        content = input;
        if input.peek(syn::token::Brace) {
            braced!(inner in input);
            content = &inner;
        }
       
        let mut object = Object::new();
        object.id = format!("object_{}", self.id);
        self.id += 1;

        loop {
            let pair = self.parse_pair(content)?;           
            object.pairs.push(pair);
            if !content.peek(Token![,]) {
                break;
            }
            content.parse::<Token![,]>()?;
        }
        
        let mut i = 0;
        fields = object.pairs.iter().map(|pair| {
            i += 1;
            let t = format!("T{}", i);
            types.push(t.clone());            
            // output
            format!("pub {}: {}", pair.key, t)
          }).collect();

        let define = format!("pub struct {}<{}> {{ {} }}", object.id, types.join(","), fields.join(", "));
        println!("XXXXXXXXXXXXXX {}", define);
        
        let v = self.append_object(object);
        return Ok(v);
    }

    // array := [value, ...]
    fn parse_array(&mut self, input: ParseStream) -> Result<Value> 
    {
        let mut array = Array::new(); 

        let inner;
        let mut content = input;

        if input.peek(syn::token::Bracket) {
            bracketed!(inner in input);
            content = &inner;
        }
        
        loop {
            let value = self.parse_value(content)?;
            array.items.push(value);
            if !content.peek(Token![,]) {
                break;                
            }
            content.parse::<Token![,]>()?;
        }
        
        let value = self.append_array(array);
        return Ok(value);
    }

    // value ：= object | array | expression
    fn parse_value(&mut self, input: ParseStream) -> Result<Value> {
        if input.peek(syn::token::Brace) {            
            return self.parse_object(input);
        }
        else if input.peek(syn::token::Bracket) {          
            return self.parse_array(input);
        }     
        return self.parse_expression(input);
    }
}

impl Parse for Json {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut json = Json::new();

        // value := object | array
        if input.peek2(syn::token::Colon) {            
            println!("XXXXXXXX parse as object");
            json.value =  json.parse_object(input)?;            
        }
        else {        
            json.value = json.parse_array(input)?;
        }     

        return Ok(json);
    }   
}
