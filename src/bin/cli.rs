use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn _prompt(question: &str) -> String {
    print!("{}", question);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] != "new" {
        println!("Использование: really_fast_api new <project_name>");
        return;
    }

    let project_name = &args[2];
    let project_path = Path::new(project_name);

    if project_path.exists() {
        eprintln!("Ошибка: Директория '{}' уже существует!", project_name);
        return;
    }

    println!("🚀 Создание нового проекта ReallyFastAPI: {}...", project_name);

    let framework_path = env!("CARGO_MANIFEST_DIR");
    
    // Используем единый стабильный шаблон (template или template_blade)
    let template_path = Path::new(framework_path).join("template_blade");
    let fallback_path = Path::new(framework_path).join("template");

    let target_template = if template_path.exists() {
        template_path
    } else {
        fallback_path
    };

    if !target_template.exists() {
        eprintln!("Ошибка: Папка шаблона не найдена в корне фреймворка!");
        return;
    }

    // Рекурсивное копирование файлов шаблона
    fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if ty.is_dir() {
                copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    copy_dir_all(&target_template, project_path).expect("Не удалось скопировать шаблон");

    // Настройка Cargo.toml
    let cargo_toml_path = project_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        let content = fs::read_to_string(&cargo_toml_path).unwrap();
        let updated_content = content
            .replace("{{project_name}}", project_name)
            .replace("{{framework_path}}", framework_path);
        fs::write(&cargo_toml_path, updated_content).unwrap();
    }

    // Автоматический запуск cargo build
    println!("📦 Запускаем `cargo build`...");
    let build_status = Command::new("cargo")
        .arg("build")
        .current_dir(project_path)
        .status();

    match build_status {
        Ok(status) if status.success() => {
            println!("\n✅ Проект '{}' успешно создан и собран!", project_name);
            println!("👉 cd {}", project_name);
            println!("🚀 cargo run");
        }
        _ => {
            eprintln!("⚠️ Проект создан, но сборка завершилась с ошибкой.");
        }
    }
}