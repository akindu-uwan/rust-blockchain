use crate::crypto::ripemd160_sha256;
// P2PKH Script
#[derive(Debug, Clone)]
pub enum ScriptItem {
    Op(String),
    Data(String),
}

pub type Script = Vec<ScriptItem>;

pub fn execute_script(script_sig: &Script, script_pubkey: &Script) -> bool {
    let mut stack: Vec<String> = Vec::new(); // Fixed: `le` -> `let`, `string` -> `String`

    let combined = [script_sig.as_slice(), script_pubkey.as_slice()].concat();

    for item in combined {
        match item {
            ScriptItem::Data(data) => stack.push(data.clone()),

            ScriptItem::Op(op) => match op.as_str() {
                "OP_DUP" => {
                    if let Some(top) = stack.last() {
                        stack.push(top.clone()); // Fixed: missing `()`
                    } else {
                        return false;
                    }
                }
                "OP_HASH160" => {
                    if let Some(top) = stack.pop() {
                        let hash = ripemd160_sha256(&top);
                        stack.push(hash);
                    } else {
                        return false;
                    }
                }
                "OP_EQUALVERIFY" => {
                    if stack.len() < 2 {
                        return false;
                    }
                    let a = stack.pop().unwrap(); // Fixed: typo `unwarap` -> `unwrap`
                    let b = stack.pop().unwrap(); // Fixed: typo `unwarap` -> `unwrap`
                    if a != b {
                        return false;
                    }
                }
                "OP_CHECKSIG" => {
                    stack.push("true".to_string()); // Fixed: incorrect syntax `to+string()` -> `to_string()`
                }
                _ => return false,
            },
        }
    }

    stack.last().map_or(false, |v| v == "true")
}
