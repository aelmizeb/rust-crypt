use rhai::{Engine, Scope};

pub fn custom_encrypt(script: &str, text: &str, key: &str) -> String {
    let engine = Engine::new();
    let mut scope = Scope::new();

    scope.push("text", text.to_string());
    scope.push("key", key.to_string());

    match engine.eval_with_scope::<String>(&mut scope, script) {
        Ok(result) => result,
        Err(err) => format!("Error: {}", err),
    }
}

pub fn custom_encrypt_file(input: &str, key: &str, script: &str) -> String {
    custom_encrypt(script, input, key)
}

pub fn custom_decrypt(input: &str, key: &str, script: &str) -> String {
    let engine = Engine::new();
    let mut scope = Scope::new();
    scope.push("text", input.to_string());
    scope.push("key", key.to_string());

    match engine.eval_with_scope::<String>(&mut scope, script) {
        Ok(result) => result,
        Err(e) => format!("❌ Script Error: {}", e),
    }
}

pub fn custom_decrypt_file(input: &str, key: &str, script: &str) -> String {
    custom_decrypt(input, key, script)
}
