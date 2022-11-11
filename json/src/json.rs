use quote::ToTokens;
use syn::{*, parse::{ParseStream, Parse}};

//------------------- JSON Syntax ------------------------------
//
// object = { pair, ...}
// pair = key : value
// key = identifier
// array  = [value, ...]
// value =  object | array | expression
// expression = string | number | identifier

pub enum ValueType {
    NULL,
    OBJECT,
    ARRAY,
    EXPRESSION,
}

pub struct Array {
    pub items: Vec<Value>,
}

pub struct Pair {
    pub key: Ident,
    pub value: Value
}

pub struct Object {    
    pub name: String,
    pub usage: i32,
    pub pairs: Vec<Pair>
}

pub struct Json {
    pub value: Value,
    pub id: i32,
    objects: Vec<Object>,
    arrays: Vec<Array>,
    expressions: Vec<Expr>,
}

pub struct Value {
    pub t: ValueType,
    pub i: usize,
}

impl Array {
    pub fn new() -> Self {
        Self {items: Vec::new() }
    }
}

impl Object {
    pub fn new() -> Self {
        Self { name: "".to_string(), pairs: Vec::new(), usage: 0 }
    }
}

fn code_gen(json: &Json, value: &Value, object_type: &String) -> String {    
    let mut code;
    let none = "".to_owned();
    match value.t {
        ValueType::OBJECT => {
            let obj = json.get_object(value);
            let mut fields = Vec::new();
            for pair in &obj.pairs {
                let v = code_gen(json, &pair.value, object_type);
                let f = format!("{}:{}", pair.key.to_string(), v);
                fields.push(f);
            }
            let name = if object_type.is_empty() { &obj.name } else { object_type };
            code = format!("{} {{ {} }}", name, fields.join(","));
        },
        ValueType::ARRAY => {         
            let array = json.get_array(value);
            let mut item_type = &none;
            // use the first item type
            if array.items.len() > 0 && matches!(array.items[0].t, ValueType::OBJECT) {
                let obj = json.get_object(&array.items[0]);
                item_type = &obj.name;
            }
            let items:Vec<_> = array.items.iter().map(|x| {
                let c = code_gen(json, x, &item_type);
                c
            }).collect();
            code = format!("[{}]", items.join(","));
        },
        ValueType::EXPRESSION => { 
            let expr = json.get_expression(value);
            code = expr.to_token_stream().to_string();            
            if code.eq("null") || code.eq("None") {                
                code = "Option::<String>::None".to_owned();
            }
        },
        ValueType::NULL => { code = "None".to_owned(); }
    }
    return  code;
}

impl Json {
    pub fn new() -> Self {
        return Self {
            value: Value { t: ValueType::NULL, i: 0 },
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
        let t = ValueType::OBJECT;
        return Value {t, i};
    }

    fn append_array(&mut self, v: Array)  -> Value
    {
        self.arrays.push(v);
        let i = self.arrays.len() - 1;
        let t = ValueType::ARRAY;
        return Value {t, i};
    }

    fn append_expression(&mut self, v: Expr)  -> Value
    {
        self.expressions.push(v);
        let i = self.expressions.len() - 1;
        let t = ValueType::EXPRESSION;
        return Value {t, i};
    }
    
    // terminal
    fn parse_expression(&mut self, input: ParseStream) -> Result<Value> 
    {   
        let expr: Expr = input.parse::<Expr>()?;
        let value = self.append_expression(expr);
        return Ok(value);
    }
    
    fn parse_pair(&mut self, input: ParseStream) -> Result<Pair> 
    { 
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
        
        content = input;
        if input.peek(syn::token::Brace) {
            braced!(inner in input);
            content = &inner;
        }
       
        let mut object = Object::new();
        object.name = format!("Object{}", self.id);
        self.id += 1;

        loop {
            let pair = self.parse_pair(content)?;           
            object.pairs.push(pair);
            if !content.peek(Token![,]) {
                break;
            }
            content.parse::<Token![,]>()?;
            if content.is_empty() {
                break;
            }
        }       

        
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
            if content.is_empty() {
                break;
            }
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

    pub fn get_pototypes(&self) -> String {
        let mut defines = Vec::new();
        defines.push("".to_owned());

        for obj in &self.objects {
            let mut i = 0;
            let mut types = Vec::new();
            let mut fields = Vec::new();
            for pair in &obj.pairs {         
                let t = format!("T{}", {i += 1; i});
                let f = format!("{}:{}", pair.key.to_string(), t);
                types.push(t);
                fields.push(f);
            }
            let define = format!("struct {}<{}> {{ {} }}", obj.name, types.join(","), fields.join(","));
            defines.push(define);
        } 
               
        let attributes = "\n#[derive(Serialize, Deserialize, Debug, Clone)]\n";

        return defines.join(attributes);    
    }

    pub fn get_code(&self) -> String {
        let name = "".to_owned();
        let code =  code_gen(self, &self.value, &name);
         // let's build our world on serde
        let import = "use serde::{Serialize, Deserialize};\n".to_owned();
        return import + &code;
    }
}

impl Parse for Json {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut json = Json::new();

        // value := object | array
        if input.peek2(syn::token::Colon) {  
            json.value =  json.parse_object(input)?;            
        }
        else {        
            json.value = json.parse_array(input)?;
        }     

        return Ok(json);
    }   
}
