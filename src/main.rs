use mlua::prelude::*;
use std::env;
use std::fs;

fn main() -> LuaResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("الاستخدام: luu [خيارات] <اسم_الملف.luu>");
        println!("الخيارات:");
        println!("  -c, --code        عرض شفرة Lua المترجمة في الطرفية");
        println!("  -o, --out <path>  تصدير شفرة Lua المترجمة إلى ملف");
        return Ok(());
    }

    let show_code = args.iter().any(|arg| arg == "-c" || arg == "--code");

    let out_path = args
        .iter()
        .position(|arg| arg == "-o" || arg == "--out")
        .and_then(|idx| args.get(idx + 1));

    let file_path = args.iter().find(|&arg| {
        !arg.starts_with('-') && arg != &args[0] && Some(arg) != out_path
    });

    let file_path = match file_path {
        Some(path) => path,
        None => {
            println!("خطأ: لم يتم تحديد ملف المدخلات.");
            return Ok(());
        }
    };

    let source_code = fs::read_to_string(file_path).expect("فشل في قراءة الملف المستهدف");

    let lua_code = tokenize_and_translate(&source_code);

    if let Some(out_file) = out_path {
        fs::write(out_file, &lua_code).expect("فشل في كتابة ملف Lua المترجم");
        println!("تم تصدير الشفرة بنجاح إلى: {}", out_file);
        return Ok(());
    }

    if show_code {
        println!("--- [شفرة Lua المترجمة] ---");
        println!("{}", lua_code);
        println!("----------------------------");
    }

    let lua = Lua::new();
    lua.load(&lua_code).exec()?;

    Ok(())
}

fn tokenize_and_translate(code: &str) -> String {
    let mut result = String::new();
    let mut current_token = String::new();
    let mut in_string = false;
    let mut string_char = ' ';
    let mut chars = code.chars().peekable();

    while let Some(ch) = chars.next() {
        if (ch == '"' || ch == '\'') && !in_string {
            flush_token(&mut current_token, &mut result);
            in_string = true;
            string_char = ch;
            result.push(ch);
            continue;
        } else if in_string {
            if ch == '\\' {
                result.push(ch);
                if let Some(next_ch) = chars.next() {
                    result.push(next_ch);
                }
            } else if ch == string_char {
                in_string = false;
                result.push(ch);
            } else {
                result.push(ch);
            }
            continue;
        }

        if ch == '،' {
            flush_token(&mut current_token, &mut result);
            result.push(',');
        } else if ch == '؛' {
            flush_token(&mut current_token, &mut result);
            result.push(';');
        } else if ch.is_whitespace() || "()[]{}=+-*/%,;:!<>".contains(ch) {
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

    let token_str = token.as_str();

    let translated = match token_str {
        "دُل" => "function".to_string(),
        "أنهِ" => "end".to_string(),
        "إذًا" => "then".to_string(),
        "وإذ" => "elseif".to_string(),
        "إذ" | "إذا" => "if".to_string(),
        "آخر" => "else".to_string(),
        "كرر" => "do".to_string(),
        "وطّن" => "local".to_string(),
        "اطبع" => "print".to_string(),
        "انتقل" => "goto".to_string(),
        "في" => "in".to_string(),

        "و" => "and".to_string(),
        "أو" => "or".to_string(),
        "لا" | "عدم" => "not".to_string(),
        "فارغ" => "nil".to_string(),

        "أصاب" => "true".to_string(),
        "أخطأ" => "false".to_string(),

        "جدول" | "جداول" => "table".to_string(),

        "لطالما" => "while".to_string(),
        "لـ" | "ل" => "for".to_string(),
        "أعد" => "do".to_string(),
        "كرر_حتى" => "repeat".to_string(),
        "حتى" => "until".to_string(),
        "اقطع" => "break".to_string(),
        "أرجع" => "return".to_string(),

        "أزواج" => "pairs".to_string(),
        "أزواج_أساس" => "ipairs".to_string(),
        "أرنِ" => "next".to_string(),
        "فكك" => "table.unpack".to_string(),

        // معالجة الأخطاء والسيطرة
        "مدفع" => "pcall".to_string(),
        "مدفع_ضاغط" => "xpcall".to_string(),
        "خطأْ" => "error".to_string(),
        "أجزم" => "assert".to_string(),

        other => {
            if other.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.') {
                other.to_string()
            } else {
                let mut romanized = String::from("v_");
                for c in other.chars() {
                    let mapped = match c {
                        'أ' | 'إ' | 'آ' | 'ا' => "a",
                        'ب' => "b",
                        'ت' | 'ة' => "t",
                        'ث' => "th",
                        'ج' => "j",
                        'ح' => "h",
                        'خ' => "kh",
                        'د' => "d",
                        'ذ' => "dh",
                        'ر' => "r",
                        'ز' => "z",
                        'س' => "s",
                        'ش' => "sh",
                        'ص' => "s",
                        'ض' => "d",
                        'ط' => "t",
                        'ظ' => "z",
                        'ع' => "a",
                        'غ' => "gh",
                        'ف' => "f",
                        'ق' => "q",
                        'ك' => "k",
                        'ل' => "l",
                        'م' => "m",
                        'ن' => "n",
                        'ه' => "h",
                        'و' => "w",
                        'ي' | 'ى' => "y",
                        '_' => "_",
                        _ => "",
                    };
                    romanized.push_str(mapped);
                }
                if romanized == "v_" {
                    let mut encoded = String::from("_v");
                    for b in other.as_bytes() {
                        encoded.push_str(&format!("_{:02X}", b));
                    }
                    encoded
                } else {
                    romanized
                }
            }
        }
    };

    output.push_str(&translated);
    token.clear();
}
