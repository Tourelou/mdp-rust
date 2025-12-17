// main.rs

mod locale;
mod parse;
mod generator;
mod clipboard;
mod open_save_ssl_cli;

use std::env;
use std::io::{self, Write};
use std::process::Command;
use std::process::ExitCode;
// Importe les types nécessaires
use locale::LangStrings;
use parse::CommandsOptions;
use generator::gen_pass;
use clipboard::send_to_clipboard;
use open_save_ssl_cli::{decrypt_via_cli, encrypt_via_cli};

// --- Logique d'Application ---

const PRG_NAME: &'static str = "mdp";
const VERSION: &'static str = "2025-12-16";
const DEFAULT_PW_LENGTH: usize = 12;
const DEFAULT_FILENAME: &'static str = "mdp.bin";

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

	let mdp_locale: LangStrings = locale::set_lang_vec();

	// ############################################################################
	// Tentative de parsing des arguments en utilisant la fonction du module 'parse'
	let config = match parse::parse_args(&mdp_locale) {
		Ok(c) => c,
		Err(e) => { 
			eprintln!("🛑 {} : {}",mdp_locale.err_cli, e);
			eprintln!("-----\n{exec_name}: {}", mdp_locale.usage);
			return ExitCode::FAILURE;
		}
	};
	let len = config.password_length.unwrap_or(DEFAULT_PW_LENGTH);

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
			let pw = gen_pass(len);
			if command_exist("pbcopy", &mdp_locale) {
				send_to_clipboard(&pw);
				println!("{} {pw} ==> Clipboard",mdp_locale.mdp_gen_str);
			}
			else { println!("{} {pw}", mdp_locale.mdp_gen_str); }
			return ExitCode::SUCCESS;
		}
		_ => { }
	}

	// ############################################################################
	// Le reste des commandes font affaire avec openssl
	if ! command_exist("openssl", &mdp_locale) {
		eprintln!("{}", mdp_locale.err_no_ssl);
		return ExitCode::FAILURE;
	}

	// Récupère la variable d'environnement "pass"
	let file_pw = match env::var("pass") {
		Ok(v) => v,
		Err(_) => {
			// La variable est absente, on demande à l'usager
			print!("Entrez le mot de passe : ");
			io::stdout().flush().unwrap(); // Force l'affichage du print! avant la lecture

			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("Erreur lors de la lecture");

			// On retire le caractère de saut de ligne (\n ou \r\n) à la fin
			input.trim().to_string()
		}
	};
	// ############################################################################
	// Identification du path du fichier mdp et chdir le cas échéant.
	let mut file_output = config.output_file.as_deref().unwrap_or(DEFAULT_FILENAME);
	let file_output_path = std::path::Path::new(file_output);

	if file_output_path.is_dir() {
		eprintln!("{}", mdp_locale.err_file_is_dir);
		return ExitCode::from(15);
	}
	if file_output.contains('/') {
		if let Some(parent) = file_output_path.parent() {
			match env::set_current_dir(parent) {
				Ok(_) => {
					if let Some(name) = file_output_path.file_name() {
						file_output = name.to_str().unwrap_or(DEFAULT_FILENAME);
					}
				}
				Err(_) => {
					eprintln!("{}", mdp_locale.err_cd_parent);
					return ExitCode::from(15);
				}
			}
		}
	}
	else {
		if let Err(_) = env::set_current_dir(exec_path) {
			eprintln!("{}", mdp_locale.err_cd_execpath);
			return ExitCode::from(15);
		}
	}

//	println!("Current directory: {:#?}", env::current_dir());

	let mdp_file_exists = file_output_path.exists();
	let pw_file_vec = if mdp_file_exists {
		decrypt_via_cli(&file_output, &file_pw).unwrap_or_else(|e| {
			eprintln!("Erreur: {}", e);
			std::process::exit(20); // Arrêt immédiat si le déchiffrement échoue
		})
	}
	else { Vec::new() }; // Fichier inexistant = liste vide

	// ############################################################################
	// Affichage synthétique des paramètres détectés
	println!("✅ Commande détectée: '{:?}' (Longueur: {}, Fichier: {})", &config.command, len, file_output);

	// Exécution de la commande selon l'énumération
	match &config.command {		// Deuxième tri, ces commandes font affaire avec un fichier => openssl
		CommandsOptions::Find(pattern) => {
			if mdp_file_exists {
				println!("Tentative de trouver le motif '{pattern}' dans le fichier '{file_output}'.");
				for ligne in pw_file_vec {
					if ligne.contains(pattern) { println!("{ligne}"); }
				}
			}
			else {
				eprintln!("Le fichier '{}' n'existe pas. Pas de recherche possible.", file_output);
				return ExitCode::FAILURE;
			}
		}
		CommandsOptions::Delete(pattern) => {
			if mdp_file_exists {
				println!("➡️ Action: Suppression du pattern '{}'.", pattern);
				_ = encrypt_via_cli("/home/danv/Desktop/dummy.bin", &pw_file_vec, &file_pw);
			}
			else {
				eprintln!("Le fichier '{}' n'existe pas. Pas de suppresion possible.", file_output);
				return ExitCode::FAILURE;
			}
		}
		CommandsOptions::Add(desc, pw) => {
			println!("➡️ Action: Ajout de '{}'. Mot de passe capturé ({} caractères).", desc, pw.len());
			println!("   Sauvegarde simulée dans le fichier : {}", file_output);
		}
		CommandsOptions::New(desc) => {
			let new_pw = generator::gen_pass(len);
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
