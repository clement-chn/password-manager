use serde::Serialize;
use serde::Deserialize;

const PASSWORD_FILE: &str = "passwords.json";

#[derive(Serialize, Deserialize)]
struct Password {
    website: String,
    password: String,
}

#[tauri::command]
fn delete_password(index: usize) -> Result<String, String> {
    println!("delete_task called with index: {}", index);

    match std::fs::read_to_string(PASSWORD_FILE) {
        Ok(content) => {
            let mut passwords: Vec<Password> = match serde_json::from_str(&content) {
                Ok(passwords) => passwords,
                Err(e) => return Err(format!("Erreur lors de la conversion JSON : {}", e)),
            };

            if index < passwords.len() {
                passwords.remove(index);

                let updated_content = match serde_json::to_string(&passwords) {
                    Ok(json) => json,
                    Err(e) => return Err(format!("Erreur lors de la conversion en JSON : {}", e)),
                };

                if let Err(e) = std::fs::write(PASSWORD_FILE, updated_content) {
                    return Err(format!("Erreur lors de l'écriture dans le fichier : {}", e));
                }

                println!("Mot de passe supprimé avec succès");
                Ok("Mot de passe supprimé avec succès".to_string())
            } else {
                Err("Index invalide".to_string())
            }
        }
        Err(e) => Err(format!("Erreur lors de la lecture du fichier : {}", e)),
    }
}

#[tauri::command]
fn get_passwords() -> Result<String, String> {
    println!("get_passwords called");

    match std::fs::read_to_string(PASSWORD_FILE) {
        Ok(content) => {
            println!("Contenu du fichier passwords.json : {}", content); // <-- Ajoute cette ligne
            Ok(content)
        },
        Err(e) => Err(format!("Erreur lors de la lecture du fichier : {}", e)),
    }
}

#[tauri::command]
fn add_password(password: String, website: String) {
    println!("website: {}", website);
    println!("password: {}", password);

    let password_struct = Password { website, password };

    let new_password_json = match serde_json::to_string(&password_struct) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Erreur lors de la conversion en JSON : {}", e);
            return;
        }
    };

    let content = std::fs::read_to_string(PASSWORD_FILE).unwrap_or_else(|_| String::new());

    let updated_content = if content.trim().is_empty() {
        format!("[{}]", new_password_json)
    } else {
        let trimmed_content = content.trim_end_matches(']');
        format!("{},{}]", trimmed_content, new_password_json)
    };

    if let Err(e) = std::fs::write(PASSWORD_FILE, updated_content) {
        eprintln!("Erreur lors de l'écriture dans le fichier : {}", e);
    } else {
        println!("Enregistrement effectué");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_password,
            get_passwords,
            delete_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}