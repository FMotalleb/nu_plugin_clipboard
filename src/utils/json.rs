use nu_protocol::ast::PathMember;
use nu_protocol::{LabeledError, Record, ShellError, Span, Value};

pub fn value_to_json_value(v: &Value) -> Result<nu_json::Value, ShellError> {
    let span = v.span();
    Ok(match v {
        Value::Bool { val, .. } => nu_json::Value::Bool(*val),
        Value::Filesize { val, .. } => nu_json::Value::I64(val.get()),
        Value::Duration { val, .. } => nu_json::Value::I64(*val),
        Value::Date { val, .. } => nu_json::Value::String(val.to_string()),
        Value::Float { val, .. } => nu_json::Value::F64(*val),
        Value::Int { val, .. } => nu_json::Value::I64(*val),
        Value::Nothing { .. } => nu_json::Value::Null,
        Value::String { val, .. } => nu_json::Value::String(val.to_string()),
        Value::Glob { val, .. } => nu_json::Value::String(val.to_string()),
        Value::CellPath { val, .. } => nu_json::Value::Array(
            val.members
                .iter()
                .map(|x| match &x {
                    PathMember::String { val, .. } => Ok(nu_json::Value::String(val.clone())),
                    PathMember::Int { val, .. } => Ok(nu_json::Value::U64(*val as u64)),
                })
                .collect::<Result<Vec<nu_json::Value>, ShellError>>()?,
        ),

        Value::List { vals, .. } => nu_json::Value::Array(json_list(vals)?),
        Value::Error { error, .. } => return Err(*error.clone()),
        Value::Closure { .. } | Value::Range { .. } => nu_json::Value::Null,
        Value::Binary { val, .. } => {
            nu_json::Value::Array(val.iter().map(|x| nu_json::Value::U64(*x as u64)).collect())
        }
        Value::Record { val, .. } => {
            let mut m = nu_json::Map::new();
            for (k, v) in &**val {
                m.insert(k.clone(), value_to_json_value(v)?);
            }
            nu_json::Value::Object(m)
        }
        Value::Custom { val, .. } => {
            let collected = val.to_base_value(span)?;
            value_to_json_value(&collected)?
        }
    })
}

pub fn json_list(input: &[Value]) -> Result<Vec<nu_json::Value>, ShellError> {
    let mut out = vec![];

    for value in input {
        out.push(value_to_json_value(value)?);
    }

    Ok(out)
}

pub fn json_to_value(v: nu_json::Value, span: Span) -> Result<Value, LabeledError> {
    Ok(match v {
        nu_json::Value::Null => Value::nothing(span),
        nu_json::Value::Bool(val) => Value::bool(val, span),
        nu_json::Value::I64(val) => Value::int(val, span),
        nu_json::Value::U64(val) => {
            if val <= i64::MAX as u64 {
                let val = val as i64;
                Value::int(val, span)
            } else {
                Value::string(format!("{}", val), span)
            }
        }
        nu_json::Value::F64(val) => Value::float(val, span),
        nu_json::Value::String(val) => Value::string(val, span),
        nu_json::Value::Array(vec) => {
            let arr: &mut Vec<Value> = &mut vec![];
            for jval in vec {
                arr.push(json_to_value(jval, span)?);
            }
            Value::list(arr.to_vec(), span)
        }
        nu_json::Value::Object(val) => {
            let mut rec = Record::new();
            for (k, v) in val {
                let value = json_to_value(v, span)?;
                rec.insert(k.clone(), value);
            }
            Value::record(rec, span)
        }
    })
}
