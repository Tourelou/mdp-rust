// locale.rs

use std::env;

const OPTIONS_FR: &str =
r#"[-f <motif>|-d <motif>|-n <desc>|-a <desc> <password>] [-l <longueur>] [fichier mdp]

Gestion des mots de passe dans un fichier crypté.

Arguments en position:
    [fichier mdp]     Fichier encrypté contenant les mots de passe. Défaut: mdp.bin.

en ligne de commande  fichier_mots_de_passe: Lu/sauvegardé dans le dossier de l'exécutable.
                      ./fichier_mots_de_passe: Lu/sauvegardé dans le dossier courrant.
                      /home/user/Documents/fichier_mots_de_passe: Lu/sauvegardé dans Documents.

Commandes: -- mutuellement exclusives
    -f/--find <motif>                      Cherche une entrée à partir d'un motif.
    -d/--del <motif>                       Supprime une entrée à partir d'un motif.
    -n/--new <description>                 Créé une nouvelle entrée à partir d'une description.
    -a/--add <description> <mot de passe>  Créé une nouvelle entrée manuellement.

    -ver/--version  Affiche un version courte/longue selon.
    -h/--help       Montre l'usage/ce message d'aide et termine.

Option:
	-l/--long <longueur>    Nombre de caracrère du mot de passe à générer. Défaut 12."#;

const OPTIONS_ES: &str =
r#"[-f <patrón>|-d <patrón>|-n <desc>|-a <desc> <contraseña>] [-l <longitud>] [archivo contras]

Gestión de contraseñas en un archivo encriptado.

Argumentos posicionales:
    [archivo contras]     Archivo encriptado que contiene las contraseñas. Predeterminado: mdp.bin.

en línea de comandos  archivo_contraseñas: Leído/guardado en la carpeta del ejecutable.
                      ./archivo_contraseñas: Leído/guardado en la carpeta actual.
                      /home/user/Documentos/archivo_contraseñas: Leído/guardado en Documentos.

Comandos: -- mutuamente excluyentes
    -f/--find <patrón>                     Busca una entrada usando un patrón.
    -d/--del <patrón>                      Elimina una entrada usando un patrón.
    -n/--new <descripción>                 Crea una nueva entrada a partir de una descripción.
    -a/--add <descripción> <contraseña>    Crea una nueva entrada manualmente.

    -ver/--version  Muestra una versión corta/larga.
    -h/--help       Muestra el uso/este mensaje de ayuda y termina.

Opción:
    -l/--long <longitud>    Número de caracteres de la contraseña a generar. Predeterminado 12."#;

const OPTIONS_EN: &str =
r#"[-f <pattern>|-d <pattern>|-n <desc>|-a <desc> <password>] [-l <length>] [pwd file]

Password management in an encrypted file.

Positional Arguments:
    [pwd file]      Encrypted file containing passwords. Default: mdp.bin.

In command line     password_file: Read/saved in the executable's directory.
                    ./password_file: Read/saved in the current directory.
                    /home/user/Documents/password_file: Read/saved in Documents.

Commands: -- mutually exclusive
    -f/--find <pattern>                    Search for an entry based on a pattern.
    -d/--del <pattern>                     Delete an entry based on a pattern.
    -n/--new <description>                 Create a new entry based on a description.
    -a/--add <description> <password>      Create a new entry manually.

    -ver/--version  Display a short/long version accordingly.
    -h/--help       Show usage/this help message and exit.

Option:
    -l/--long <length>      Number of characters for the password to generate. Default 12."#;

#[derive(Debug)]
pub struct LangStrings {
	pub err_opt_l_1fois: &'static str,
	pub err_opt_l_int: &'static str,
	pub err_opt_l_bound: &'static str,
	pub manque_args: &'static str,
	pub err_bad_arg: &'static str,
	pub err_valid_args: &'static str,
	pub err_cli: &'static str,
	pub err_which: &'static str,
	pub err_no_ssl: &'static str,
	pub err_file_is_dir: &'static str,
	pub err_cd_parent: &'static str,
	pub err_cd_execpath: &'static str,
	pub err_err: &'static str,
	pub err_keyboard: &'static str,
	pub err_index_too_big: &'static str,
	pub comm_mut_excl: &'static str,
	pub mdp_gen_str: &'static str,
	pub enter_encryp_pw: &'static str,
	pub find_header: &'static str,
	pub find_no_file: &'static str,
	pub no_match: &'static str,
	pub index_to_clip: &'static str,
	pub usage: &'static str,
	pub options: &'static str,
	pub ver: &'static str,
	pub ver_desc: &'static str,
}

pub const LANG_FR: LangStrings = LangStrings {
	err_opt_l_1fois: "L'option -l/--long ne peut être spécifiée qu'une seule fois.",
	err_opt_l_int: "La longueur doit être un nombre entier valide.",
	err_opt_l_bound: "La longueur doit être comprise entre 8 et 32 caractères.",
	manque_args: "Arguments nécessaire manquant !!",
	err_bad_arg: "Argument non reconnu ou mal placé",
	err_valid_args: "La ligne de commande doit commencer par une option valide:\n\t\t==> (-f/--find, -d/--del, -a/--add, -n/--new, ou -l/--long) ou être vide.",
	err_cli: "Erreur de ligne de commande",
	err_which: "Erreur de la command which.",
	err_no_ssl: "La commande 'openssl' est inaccessible.",
	err_file_is_dir: "Erreur: le fichier proposé est un répertoire.",
	err_cd_parent: "Erreur: Ne peux 'chdir' vers le dossier parent.",
	err_cd_execpath: "Erreur: Ne peux 'chdir' vers le dossier de l'exécutable.",
	err_err: "Erreur:",
	err_keyboard: "Erreur lors de la saisi au clavier.",
	err_index_too_big: "Erreur : '{1}' est plus grand que le nombre d'entrée.",
	comm_mut_excl: "Les commandes sont mutuellement exclusives.",
	mdp_gen_str: "Mot de passe généré:",
	enter_encryp_pw: "Entrez le mot de passe d'encryption : ",
	find_header: "Tentative de trouver le motif '{1}' dans le fichier '{2}'.",
	find_no_file: "Le fichier '{1}' n'existe pas. Pas de recherche possible.",
	no_match: "Rien trouvé pour le motif '{1}'",
	index_to_clip: "Quelle entrée ira vers le presse-papier ? ",
	usage: "[-f <motif>|-d <motif>|-n <desc>|-a <desc> <password>] [-l <longueur>] [fichier mdp]",
	options: OPTIONS_FR,
	ver: "version",
	ver_desc: "Gestion de mots de passe en CLI, version",
};

pub const LANG_ES: LangStrings = LangStrings {
	err_opt_l_1fois: "La opción -l/--long solo se puede especificar una vez.",
	err_opt_l_int: "La longitud debe ser un número entero válido.",
	err_opt_l_bound: "La longitud debe estar entre 8 y 32 caracteres.",
	manque_args: "Argumentos necesarios faltantes !!",
	err_bad_arg: "Argumento no reconocido o mal colocado",
	err_valid_args: "La línea de comandos debe comenzar con una opción válida:\n\t\t==> (-f/--find, -d/--del, -a/--add, -n/--new, o -l/--long) o estar vacía.",
	err_cli: "Error de línea de comandos",
	err_which: "Error del comando 'which'.",
	err_no_ssl: "El comando 'openssl' es inaccesible.",
	err_file_is_dir: "Error: el archivo propuesto es un directorio.",
	err_cd_parent: "Error: No se puede 'chdir' al directorio padre.",
	err_cd_execpath: "Error: No se puede 'chdir' al directorio del ejecutable.",
	err_err: "Error:",
	err_keyboard: "Error durante la entrada por teclado.",
	err_index_too_big: "Error: '{1}' es mayor que el número de entradas.",
	comm_mut_excl: "Los comandos son mutuamente excluyentes.",
	mdp_gen_str: "Contraseña generada:",
	enter_encryp_pw: "Ingrese la contraseña de cifrado: ",
	find_header: "Intentando buscar el patrón '{1}' en el archivo '{2}'.",
	find_no_file: "El archivo '{1}' no existe. Búsqueda no posible.",
	no_match: "No se encontró nada para el patrón '{1}'",
	index_to_clip: "¿Qué entrada desea copiar al portapapeles? ",
	usage: "[-f <patrón>|-d <patrón>|-n <desc>|-a <desc> <contraseña>] [-l <longitud>] [archivo contras]",
	options: OPTIONS_ES,
	ver: "versión",
	ver_desc: "Gestión de contraseñas en CLI, versión",
};

pub const LANG_EN: LangStrings = LangStrings {
	err_opt_l_1fois: "The -l/--long option can only be specified once.",
	err_opt_l_int: "The length must be a valid integer.",
	err_opt_l_bound: "The length must be between 8 and 32 characters.",
	manque_args: "Missing required arguments !!",
	err_bad_arg: "Unrecognized or misplaced argument",
	err_valid_args: "The command line must start with a valid option:\n\t\t==> (-f/--find, -d/--del, -a/--add, -n/--new, or -l/--long) or be empty.",
	err_cli: "Command line error",
	err_which: "Error with the 'which' command.",
	err_no_ssl: "The 'openssl' command is inaccessible.",
	err_file_is_dir: "Error: the proposed file is a directory.",
	err_cd_parent: "Error: Cannot 'chdir' to the parent directory.",
	err_cd_execpath: "Error: Cannot 'chdir' to the executable's directory.",
	err_err: "Error:",
	err_keyboard: "Error during keyboard input.",
	err_index_too_big: "Error: '{1}' is greater than the number of entries.",
	comm_mut_excl: "Commands are mutually exclusive.",
	mdp_gen_str: "Generated password:",
	enter_encryp_pw: "Enter the encryption password: ",
	find_header: "Attempting to find the pattern '{1}' in file '{2}'.",
	find_no_file: "File '{1}' does not exist. Search not possible.",
	no_match: "No matches found for pattern '{1}'",
	index_to_clip: "Which entry should go to the clipboard? ",
	usage: "[-f <pattern>|-d <pattern>|-n <desc>|-a <desc> <password>] [-l <length>] [pwd file]",
	options: OPTIONS_EN,
	ver: "version",
	ver_desc: "CLI password management, version",
};

pub fn set_lang_vec() -> LangStrings {
	match get_system_lang().as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	}
}

fn get_system_lang() -> String {
	let raw_lang = std::env::var("LC_ALL")
		.or_else(|_| env::var("LANG"))
		.or_else(|_| env::var("LANGUAGE"))
		.unwrap_or_else(|_| "en".to_string()); // Langue par défaut (anglais)

	// Extraire uniquement le code de langue avant le premier '_'
	let lang_code = raw_lang.split('_').next().unwrap_or(&raw_lang);
	lang_code.to_string() // Retourne "fr" au lieu de "fr_CA.UTF-8"
}
