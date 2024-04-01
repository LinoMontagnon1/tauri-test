// Inclui as dependências necessárias.
use rusqlite::{params, Connection, Result};
use bcrypt::{DEFAULT_COST, hash, verify};
use tauri::Manager;
use csv::Writer;
use std::fs::File;

// Comando para cumprimentar que já existia.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Função para criar um hash de uma senha
fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// Função para verificar uma senha contra um hash
fn verify_password(password: &str, password_hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, password_hash)
}

// Comando para realizar o login.
#[tauri::command]
fn login(username: &str, password: &str) -> Result<bool> {
    let conn = Connection::open("meu_banco.db")?;

    let mut stmt = conn.prepare("SELECT password_hash FROM users WHERE username = ?1")?;
    let mut rows = stmt.query(params![username])?;

    if let Some(row) = rows.next()? {
        let password_hash: String = row.get(0)?;
        // Verifica a senha fornecida com o hash do banco de dados.
        verify_password(password, &password_hash).or(Err(rusqlite::Error::from(rusqlite::error::Error::InvalidQuery)))
    } else {
        // Nome de usuário não encontrado.
        Ok(false)
    }
}

// Comando para registrar um novo usuário.
#[tauri::command]
fn register(username: &str, password: &str) -> Result<()> {
    let conn = Connection::open("meu_banco.db")?;
    let password_hash = hash_password(password)?;

    conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
        params![username, password_hash],
    )?;
    Ok(())
}

// Comando para exportar todos os usuários para um arquivo CSV.
#[tauri::command]
fn export_users_to_csv() -> Result<String, String> {
    let conn = Connection::open("meu_banco.db").map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT username FROM users").map_err(|e| e.to_string())?;
    let users = stmt.query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, rusqlite::Error>>()
        .map_err(|e| e.to_string())?;

    let file_path = "users.csv";
    let file = File::create(file_path).map_err(|e| e.to_string())?;
    let mut wtr = Writer::new(file);

    for user in users {
        wtr.write_record(&[user]).map_err(|e| e.to_string())?;
    }
    wtr.flush().map_err(|e| e.to_string())?;

    Ok(file_path.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, login, register, export_users_to_csv]) // Adicione `export_users_to_csv` aqui
        .setup(|_app| {
            // Este bloco de configuração é executado antes da janela abrir.

            // Criação e configuração inicial do banco de dados.
            let conn = Connection::open("meu_banco.db").expect("falha ao abrir banco de dados");
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY,
                    username TEXT NOT NULL UNIQUE,
                    password_hash TEXT NOT NULL
                 )",
                [],
            ).expect("falha ao criar tabela de usuários");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
