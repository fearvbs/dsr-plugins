use std::io::{self, Read, Write};
use base64::{Engine as _, engine::general_purpose::STANDARD};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    // Поддержка --describe
    if args.len() > 1 && args[1] == "--describe" {
        print_describe();
        return Ok(());
    }
    
    // Определяем режим
    let decode = args.iter().any(|arg| arg == "--decode" || arg == "-d");
    
    // Получаем данные: либо из аргументов, либо из stdin
    let input_data = if args.len() > 2 && !args[2].starts_with('-') {
        // Данные переданы как аргумент (например: dsr do base64 encode "hello")
        args[2].as_bytes().to_vec()
    } else {
        // Читаем из stdin
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input)?;
        input
    };
    
    // Обрабатываем
    if decode {
        let cleaned = input_data.iter()
            .filter(|&&c| !c.is_ascii_whitespace())
            .copied()
            .collect::<Vec<u8>>();
        
        let decoded = STANDARD.decode(&cleaned)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        io::stdout().write_all(&decoded)?;
    } else {
        let encoded = STANDARD.encode(&input_data);
        io::stdout().write_all(encoded.as_bytes())?;
    }
    
    Ok(())
}

fn print_describe() {
    let json = r#"{
        "name": "base64",
        "version": "0.2.0",
        "actions": [
            {
                "name": "encode",
                "description": "Encode data to Base64",
                "usage": "encode [text]"
            },
            {
                "name": "decode",
                "description": "Decode Base64 to original data",
                "usage": "decode [text]"
            }
        ]
    }"#;
    println!("{}", json);
}