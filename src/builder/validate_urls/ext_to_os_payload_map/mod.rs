use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub enum Value {
    String(String),
    StringList(Vec<String>),
}

pub static WIN_CONFIG: Lazy<HashMap<String, Value>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "dropper_exts".to_string(),
        Value::StringList(vec![
            "pwsh".to_string(),
            "ps1".to_string(),
            "bat".to_string(),
            "vbs".to_string(),
        ]),
    );
    m
});

pub static LIN_CONFIG: Lazy<HashMap<String, Value>> = Lazy::new(|| {
   let mut m = HashMap::new();
    m.insert("dropper_exts".to_string(), Value::StringList(vec![
        "sh".to_string(),
        "zsh".to_string(),
        "csh".to_string(),
        "cksh".to_string(),
        "esh".to_string(),
        "bash".to_string()
    ]),
    );
    m
});

pub static MAC_CONFIG: Lazy<HashMap<String, Value>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("dropper_exts".to_string(), Value::StringList(vec![
     "sh".to_string(),
     "zsh".to_string(),
     "csh".to_string(),
     "cksh".to_string(),
     "esh".to_string(),
     "bash".to_string()
    ])
    );
    m
});

pub static LANGUAGE_EXTS: Lazy<HashMap<String, Value>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("deno".to_string(), Value::StringList(vec![
        "ts".to_string(),
        "tsx".to_string(),
        "js".to_string()
    ])
    );
    m.insert("dlang".to_string(), Value::StringList(vec![
        "d".to_string()
    ]));
    m.insert("vlang".to_string(), Value::StringList(vec![
        "v".to_string()
    ]));
    m.insert("python".to_string(), Value::StringList(vec![
        "py".to_string(),
        "pyzw".to_string(),
        "pyz".to_string(),
        "pyc".to_string()
    ]));
    m.insert("php".to_string(), Value::StringList(vec![
        "php".to_string(),
        "php4".to_string(),
        "php5".to_string(),
        "php6".to_string(),
        "php7".to_string(),
        "php8".to_string(),
        "phtml".to_string(),
        "txt".to_string(),
        "html".to_string()
    ]));
    m.insert("tcl".to_string(), Value::StringList(vec![
        "tcl".to_string()
    ]));
    m.insert("crystal".to_string(), Value::StringList(vec![
        "cr".to_string()
    ]));
    m.insert("julia".to_string(), Value::StringList(vec![
        "jl".to_string()
    ]));
    m.insert("rlang".to_string(), Value::StringList(vec![
        "r".to_string()
    ]));
    m.insert("bun".to_string(), Value::StringList(vec![
        "js".to_string(),
        "ts".to_string(),
        "tsx".to_string()
    ]));
    m.insert("fsharp".to_string(), Value::StringList(vec![
        "fsx".to_string(),
        "f".to_string(),
        "fs".to_string()
    ]));
    m.insert("dart".to_string(), Value::StringList(vec![
        "dart".to_string()
    ]));
    m.insert("golang".to_string(), Value::StringList(vec![
        "go".to_string()
    ]));
    m
});