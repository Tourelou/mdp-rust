// main.rs

mod locale;
mod parse;
mod generator;
mod clipboard;
mod get_pw;
mod openssl_cli;
mod actions;

use std::env;
use std::process::Command;
use std::path::Path;
use std::process::ExitCode;
// Importe les types nécessaires
use locale::LangStrings;
use parse::CommandsOptions;
use generator::gen_pass;
use clipboard::send_to_clipboard;
use openssl_cli::{decrypt_via_cli, encrypt_via_cli};

// --- Logique d'Application ---

const PRG_NAME: &'static str = "mdp";
const VERSION: &'static str = "2025-12-20";
const DEFAULT_PW_LENGTH: usize = 12;
const DEFAULT_FILENAME: &'static str = "mdp.bin";

#[derive(Debug)]
#[allow(dead_code)]
pub struct AppData {
	app_locale: LangStrings,
	app_pw_len: usize,
	app_encryp_pass: String,
	app_line_vec: Vec<String>,
}

pub fn command_exist(cmd: &str, locale: &LangStrings) -> bool {
	let status = Command::new("which")
		.arg(cmd)
		.stdout(std::process::Stdio::null()) // Redirige le chemin trouvé vers /dev/null
		.stderr(std::process::Stdio::null()) // Redirige les erreurs vers /dev/null
		.status()
		.expect(locale.err_which);

	if status.success() { return true; }
	else { return false; }
}

pub fn open_mdp_file() {

}

// --- Fonction Principale ---

pub fn main() -> ExitCode {
	// ############################################################################
	// Récupère le nom et le path de l'exécutable
	let exec_full_path = match env::current_exe() {
		Ok(path_buf) => path_buf,
		Err(_) => { return ExitCode::FAILURE; },
	};

	let exec_name = exec_full_path.file_name()
								.and_then(|n| n.to_str())
								.unwrap_or(PRG_NAME);

	let exec_path = exec_full_path.parent()
									.and_then(|p| p.to_str())
									.unwrap_or(".");

	let app_locale = locale::set_lang_vec();
	// ############################################################################
	// Tentative de parsing des arguments en utilisant la fonction du module 'parse'
	let config = match parse::parse_args(&app_locale) {
		Ok(c) => c,
		Err(e) => { 
			eprintln!("🛑 {} : {}",app_locale.err_cli, e);
			eprintln!("-----\n{exec_name}: {}", app_locale.usage);
			return ExitCode::FAILURE;
		}
	};
	let app_pw_len = config.password_length.unwrap_or(DEFAULT_PW_LENGTH);

	match &config.command {		// Premier tri, ces commandes n'ont pas besoin de openssl
		CommandsOptions::Help(message) => {
			println!("{exec_name}: {message}");
			return ExitCode::SUCCESS;
		}
		CommandsOptions::Version(message) => {
			println!("{exec_name}: {message} {VERSION}");
			return ExitCode::SUCCESS;
		}
		CommandsOptions::GeneratePassword => {
			let pw = gen_pass(app_pw_len);
			if command_exist("pbcopy", &app_locale) {
				send_to_clipboard(&pw);
				println!("{} {pw} ==> Clipboard",app_locale.mdp_gen_str);
			}
			else { println!("{} {pw}", app_locale.mdp_gen_str); }
			return ExitCode::SUCCESS;
		}
		_ => { }
	}

	// ############################################################################
	// Le reste des commandes font affaire avec openssl
	if ! command_exist("openssl", &app_locale) {
		eprintln!("{}", app_locale.err_no_ssl);
		return ExitCode::FAILURE;
	}

	// ############################################################################
	// Identification du path du fichier mdp et chdir le cas échéant.
	let mut file_output = config.output_file.as_deref().unwrap_or(DEFAULT_FILENAME);
	let mut file_output_path = Path::new(file_output);

	if file_output_path.is_dir() {
		eprintln!("{}", app_locale.err_file_is_dir);
		return ExitCode::from(15);
	}
	if file_output.contains('/') {
		if let Some(parent) = file_output_path.parent() {
			match env::set_current_dir(parent) {
				Ok(_) => {
					if let Some(name) = file_output_path.file_name() {
						file_output = name.to_str().unwrap_or(DEFAULT_FILENAME);
						file_output_path = Path::new(file_output);
					}
				}
				Err(_) => {
					eprintln!("{}", app_locale.err_cd_parent);
					return ExitCode::from(15);
				}
			}
		}
	}
	else {
		if let Err(_) = env::set_current_dir(exec_path) {
			eprintln!("{}", app_locale.err_cd_execpath);
			return ExitCode::from(15);
		}
	}
	let mdp_full_path = env::current_dir().unwrap()
							.join(file_output)
							.to_string_lossy().into_owned();

	let mdp_file_exists = file_output_path.exists();
	let mut app_encryp_pass = String::new();

	let app_line_vec = if mdp_file_exists {
		// Récupère la variable d'environnement "pass"
		app_encryp_pass = match env::var("pass") {
			Ok(v) => v,
			Err(_) => get_pw!(app_locale.enter_encryp_pw),
		};
		decrypt_via_cli(&file_output, &app_encryp_pass)
									.unwrap_or_else(|e| {
			eprintln!("{} {}", app_locale.err_err, e);
			std::process::exit(20); // Arrêt immédiat si le déchiffrement échoue
		})
	}
	else { Vec::new() }; // Fichier inexistant = liste vide

	let mut app_data = AppData {app_locale, app_pw_len, app_encryp_pass, app_line_vec};
	// ############################################################################
	// Exécution de la commande selon l'énumération
	match &config.command {		// Deuxième tri, ces commandes font affaire avec un fichier => openssl
		CommandsOptions::Find(pattern) => {
			if mdp_file_exists {
				println!("{}", app_data.app_locale.find_header
									.replace("{1}", pattern)
									.replace("{2}", &mdp_full_path));
				actions::find(&pattern, &app_data);
			}
			else {
				eprintln!("{}", app_data.app_locale.find_no_file.replace("{1}", &mdp_full_path));
				return ExitCode::FAILURE;
			}
		}
		CommandsOptions::Delete(pattern) => {
			if mdp_file_exists {
				println!("{}", app_data.app_locale.del_header
									.replace("{1}", pattern)
									.replace("{2}", &mdp_full_path));
				if actions::del(&pattern, &mut app_data) {
					encrypt_via_cli(&file_output,
								&app_data.app_line_vec,
								&app_data.app_encryp_pass).unwrap_or_else(|e| {
					eprintln!("{} {}", app_data.app_locale.err_err, e);
					std::process::exit(20); // Arrêt immédiat si le déchiffrement échoue
					})
				}
			}
			else {
				eprintln!("{}", app_data.app_locale.del_no_file.replace("{1}", &mdp_full_path));
				return ExitCode::FAILURE;
			}
		}
		CommandsOptions::Add(desc, pw) => {
			println!("➡️ Action: Ajout de '{}'. Mot de passe capturé ({} caractères).", desc, pw.len());
			println!("   Sauvegarde simulée dans le fichier : {}", file_output);
		}
		CommandsOptions::New(desc) => {
			let new_pw = generator::gen_pass(app_pw_len);
			println!("➡️ Action: Ajout de '{}'. Mot de passe généré {}.", desc, new_pw);
			println!("   Sauvegarde simulée dans le fichier : {}", file_output);
		}
		CommandsOptions::None => {
			eprintln!("🛑 Erreur interne: Commande non définie.");
			return ExitCode::FAILURE;
		}
		_ => { }
	}
	return ExitCode::SUCCESS;
}
