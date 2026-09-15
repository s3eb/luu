use mlua::prelude::*;
use std::env;
use std::fs;

fn main() -> LuaResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("الاستخدام: luu <اسم_الملف.luu>");
        return Ok(());
    }

    let file_path = &args[1];
    let source_code = fs::read_to_string(file_path)
        .expect("فشل في قراءة الملف المستهدف");

    let lua_code = tokenize_and_translate(&source_code);

    let lua = Lua::new();
    lua.load(&lua_code).exec()?;

    Ok(())
}

fn tokenize_and_translate(code: &str) -> String {
    let mut result = String::new();
    let mut current_token = String::new();
    let mut in_string = false;
    let mut string_char = ' ';

    for ch in code.chars() {
        if (ch == '"' || ch == '\'') && !in_string {
            flush_token(&mut current_token, &mut result);
            in_string = true;
            string_char = ch;
            result.push(ch);
            continue;
        } else if in_string && ch == string_char {
            in_string = false;
            result.push(ch);
            continue;
        }

        if in_string {
            result.push(ch);
            continue;
        }

        if ch.is_whitespace() || "()[]{}=+-*/%,;:!<>".contains(ch) {
            flush_token(&mut current_token, &mut result);
            result.push(ch);
        } else {
            current_token.push(ch);
        }
    }
    flush_token(&mut current_token, &mut result);

    result
}

fn flush_token(token: &mut String, output: &mut String) {
    if token.is_empty() {
        return;
    }

    let translated = match token.as_str() {
        // الكلمات المفتاحية
        "دُل" => "function".to_string(),
        "أنهِ" => "end".to_string(),
        "إذًا" => "then".to_string(),
        "وإذ" => "elseif".to_string(),
        "إذ" => "if".to_string(),
        "آخر" => "else".to_string(),
        "كرر" => "do".to_string(),
        "وطّن" => "local".to_string(),
        "اطبع" => "print".to_string(),
        
        // الأرقام والرموز اللاتينية تبقى كما هي
        other => {
            if other.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                other.to_string()
            } else {
                // تحويل المتغيرات العربية إلى معرّفات مقبولة لدى Lua
                let mut encoded = String::from("_v");
                for b in other.as_bytes() {
                    encoded.push_str(&format!("_{:02X}", b));
                }
                encoded
            }
        }
    };

    output.push_str(&translated);
    token.clear();
}
