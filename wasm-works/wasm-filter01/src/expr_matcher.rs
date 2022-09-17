use std::collections::HashMap;
use evalexpr::{context_map, ContextWithMutableFunctions,
               eval_boolean_with_context,
               EvalexprError, EvalexprResult, Function, HashMapContext, Value};
use regex::{RegexBuilder};

fn bool_regex(argument: &Value) -> Result<Value, EvalexprError> {
    let arguments = argument.as_tuple()?;
    if let (Value::String(regex_st), Value::String(val_st)) = (&arguments[0], &arguments[1]) {
        let re_result = RegexBuilder::new(regex_st.as_str()).build();
        if let Ok(regex) = re_result {
            return Ok(Value::from(regex.is_match(val_st)));
        } else {
            return Ok(Value::from(false));
        }
    }
    Err(EvalexprError::expected_string(Value::String("Bool Regex Failed".to_string())))
}

fn expr_match(argument: &Value) -> Result<Value, EvalexprError> {
    let arguments = argument.as_tuple()?;
    if let (Value::String(regex_st), Value::String(val_st)) = (&arguments[0], &arguments[1]) {
        if regex_st == val_st {
            return Ok(Value::from(true));
        } else {
            return Ok(Value::from(false));
        }
    }
    Err(EvalexprError::expected_string(
        Value::String("Match Failed".to_string()))
    )
}

fn length(argument: &Value) -> Result<Value, EvalexprError> {
    if argument.is_string() {
        if let Ok(value) = argument.as_string() {
            return Ok(Value::Int(value.len() as i64));
        } else {
            return Ok(Value::Int(-1 as i64));
        }
    }
    if argument.is_tuple() {
        if let Ok(value) = argument.as_tuple() {
            return Ok(Value::Int(value.len() as i64));
        } else {
            return Ok(Value::Int(-1 as i64));
        }
    }

    Err(EvalexprError::expected_string(
        Value::String("Length Failed".to_string()))
    )
}

fn index(argument: &Value) -> Result<Value, EvalexprError> {
    if let Ok(arguments) = argument.as_tuple() {
        if let (Value::Tuple(tuple), Value::Int(index)) = (&arguments[0], &arguments[1]) {
            let tup = tuple.to_vec();
            let i = *index as usize;
            if let Some(val) = tup.get(i) {
                return Ok(val.clone());
            } else {
                return Ok(Value::from("".to_string()));
            }
        }
    }

    Err(EvalexprError::expected_string(
        Value::String("Index Failed".to_string()))
    )
}

fn lowercase(argument: &Value) -> Result<Value, EvalexprError> {
    if let Ok(value) = argument.as_string() {
        return Ok(Value::String(value.to_lowercase()));
    }
    Err(EvalexprError::expected_string(
        Value::String("Lowercase Failed".to_string()))
    )
}

fn split(argument: &Value) -> Result<Value, EvalexprError> {
    let arguments = argument.as_tuple()?;
    if let (Value::String(sent), Value::String(token)) = (&arguments[0], &arguments[1]) {
        let splits: Vec<&str> = sent.split(token).collect();
        let vals: Vec<Value> = splits.iter().map(|&v| Value::from(v)).collect();
        return Ok(Value::from(vals));
    }
    Err(EvalexprError::expected_string(
        Value::String("Split Failed".to_string()))
    )
}


pub fn get_expression_context() -> HashMapContext {
    let header_context = context_map! {
        "bool_regex" => Function::new(|argument| bool_regex(argument) ),
        "match" => Function::new(|argument| expr_match(argument)),
        "len" => Function::new(|argument| length(argument)),
        "lowercase" => Function::new(|argument| lowercase(argument)),
        // "head" // to be added via context function
        "split" => Function::new(|argument| split(argument)),
        "index" => Function::new(|argument| index(argument)),
    }.unwrap();
    header_context
}

pub fn bool_check(exp: &str, context: &HashMapContext) -> EvalexprResult<bool> {
    eval_boolean_with_context(exp, context)
}

fn get_hash_map(key_val: &Vec<(String, String)>) -> HashMap<String, String> {
    key_val.iter()
        .map(|(k, v)| (k.to_lowercase().to_string(), v.to_string()))
        .collect::<HashMap<_, _>>()
}

pub fn append_context_vec(
    header_context: &mut HashMapContext,
    vec: Vec<(String, String)>,
) -> &mut HashMapContext {
    let func = move |argument: &Value| {
        if let Ok(key) = argument.as_string() {
            let low_keys = get_hash_map(&vec);
            if let Some(value) = low_keys.get(&key) {
                return Ok(Value::from(value.to_string()));
            } else {
                return Ok(Value::from("".to_string()));
            }
        }
        Err(EvalexprError::expected_string(
            Value::String("Head Failed".to_string()))
        )
    };
    let _ = header_context.set_function(
        "head".to_string(),
        Function::new(func),
    );
    return header_context;
}
