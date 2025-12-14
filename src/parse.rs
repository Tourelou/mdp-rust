// parse.rs

use std::env;

use crate::locale::LangStrings;

// --- 1. Structures de Données ---

// Définition des actions possibles (mutuellement exclusives)
#[derive(Debug)]
pub enum CommandsOptions {
	Help(String),
	Version(String),
	Find(String),
	Delete(String),
	New(String),
	Add(String, String), // Description, Mot de passe
	GeneratePassword, 
	None,
}

// Structure pour stocker la configuration
#[derive(Debug)]
pub struct Config {
	pub command: CommandsOptions,
	pub password_length: Option<usize>,
	pub output_file: Option<String>,
}

// Helper pour déterminer si une commande a déjà été détectée
impl CommandsOptions {
	fn is_some(&self) -> bool {
		!matches!(self, CommandsOptions::None)
	}
}

// --- 2. Logique de Parsing ---

/// Parse les arguments de la ligne de commande avec des contraintes d'ordre strictes.
pub fn parse_args(mdp_locale: &LangStrings) -> Result<Config, String> {
	let args: Vec<String> = env::args().collect();

	// Règle 0: Ligne vide (un seul argument, le nom du programme)
	if args.len() == 1 {
		return Ok(Config {
			command: CommandsOptions::GeneratePassword,
			password_length: None,
			output_file: None,
		});
	}

	// Règle 1: On passe toute la ligne pour help ou version
	let mut command: CommandsOptions = CommandsOptions::None;

	for i in 1..args.len() {
		match args[i].as_str() {
			"-h" => { command = CommandsOptions::Help(mdp_locale.usage.to_string()); },
			"--help" => { command = CommandsOptions::Help(mdp_locale.options.to_string()); },
			"-ver" => { command = CommandsOptions::Version(mdp_locale.ver.to_string()); },
			"--version" => { command = CommandsOptions::Version(mdp_locale.ver_desc.to_string()); },
			_ => break
		}
		return Ok(Config {
			command,
			password_length: None,
			output_file: None,
		});
	}
	if args[1].chars().nth(0) != Some('-') {
		return Err(mdp_locale.err_valid_args.to_string());
	}
	let mut password_length: Option<usize> = None;
	let mut indices_utilises = vec![0];

	let max_index = args.len();
	let mut current_index = 1;

	while current_index < max_index {
		
		if indices_utilises.contains(&current_index) {
			current_index += 1;
			continue;
		}

		let arg = &args[current_index];

		match arg.as_str() {
			// --- Option de Longueur (-l/--long) ---
			"-l" | "--long" => {
				if current_index + 1 >= max_index { 
					return Err(format!("-l/--long : {}", mdp_locale.manque_args)); 
				}
				
				let len_str = &args[current_index + 1];
				match len_str.parse::<usize>() {
					Ok(len) => {
						if password_length.is_some() {
							return Err(mdp_locale.err_opt_l_1fois.to_string());
						}
						password_length = Some(len);
						if password_length < Some(8) || password_length > Some(32) {
							return Err(mdp_locale.err_opt_l_bound.to_string());
						}
						indices_utilises.push(current_index);
						indices_utilises.push(current_index + 1);
						current_index += 2;
					},
					Err(_) => return Err(mdp_locale.err_opt_l_int.to_string()),
				}
			}

			// --- Options principales (mutuellement exclusives) ---
			"-f" | "--find" => {
				if command.is_some() { return Err(mdp_locale.comm_mut_excl.to_string()); }
				if current_index + 1 >= max_index { return Err(format!("-f/--find : {}", mdp_locale.manque_args)); }
				command = CommandsOptions::Find(args[current_index + 1].clone());
				indices_utilises.push(current_index);
				indices_utilises.push(current_index + 1);
				current_index += 2;
			}
			"-d" | "--del" => {
				if command.is_some() { return Err(mdp_locale.comm_mut_excl.to_string()); }
				if current_index + 1 >= max_index { return Err(format!("-d/--del : {}", mdp_locale.manque_args)); }
				command = CommandsOptions::Delete(args[current_index + 1].clone());
				indices_utilises.push(current_index);
				indices_utilises.push(current_index + 1);
				current_index += 2;
			}
			"-a" | "--add" => {
				if command.is_some() { return Err(mdp_locale.comm_mut_excl.to_string()); }
				if current_index + 2 >= max_index { return Err(format!("-a/--add : {}", mdp_locale.manque_args)); }
				command = CommandsOptions::Add(args[current_index + 1].clone(), args[current_index + 2].clone());
				indices_utilises.push(current_index);
				indices_utilises.push(current_index + 1);
				indices_utilises.push(current_index + 2);
				current_index += 3;
			}
			"-n" | "--new" => {
				if command.is_some() { return Err(mdp_locale.comm_mut_excl.to_string()); }
				if current_index + 1 >= max_index { return Err(format!("-n/--new : {}", mdp_locale.manque_args)); }
				command = CommandsOptions::New(args[current_index + 1].clone());
				indices_utilises.push(current_index);
				indices_utilises.push(current_index + 1);
				current_index += 2;
			}
			
			_ => {
				current_index += 1;
			}
		}
	}

	// 2. Détecter le nom de fichier optionnel (DOIT être le dernier argument)
	let mut output_file: Option<String> = None;
	let dernier_index = max_index - 1;
	
	if !indices_utilises.contains(&dernier_index) && !args[dernier_index].starts_with('-') {
		output_file = Some(args[dernier_index].clone());
		indices_utilises.push(dernier_index);
	}
	
	// 3. Validation de l'intégrité
	for i in 1..max_index {
		if !indices_utilises.contains(&i) {
			return Err(format!("{} : {}",mdp_locale.err_bad_arg, args[i]));
		}
	}

	// 4. Gestion de la commande par défaut
	if let CommandsOptions::None = command {
		if password_length.is_some() || output_file.is_some() {
			command = CommandsOptions::GeneratePassword;
		}
		else {
			 return Err(mdp_locale.err_valid_args.to_string());
		}
	}

	Ok(Config {
		command,
		password_length,
		output_file,
	})
}
