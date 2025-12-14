// locale.rs

use std::env;

const OPTIONS_FR: &str =
r#"[-f <motif>|-d <motif>|-n <desc>|-a <desc> <password>] [-l <longueur>] [fichier mdp]

Gestion des mots de passe dans un fichier crypté.

Arguments en position:
	[fichier mdp]     Fichier encrypté contenant les mots de passe. Défaut: mdp.bin.

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
r#""#;

const OPTIONS_EN: &str =
r#""#;

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
	pub comm_mut_excl: &'static str,
	pub mdp_gen_str: &'static str,
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
	err_bad_arg: "Argument non reconnu ou mal placé.",
	err_valid_args: "La ligne de commande doit commencer par une option valide:\n\t\t==> (-f/--find, -d/--del, -a/--add, -n/--new, ou -l/--long) ou être vide.",
	err_cli: "Erreur de ligne de commande.",
	err_which: "Erreur de la command which.",
	err_no_ssl: "La commande 'openssl' est inaccessible.",
	err_file_is_dir: "Erreur: le fichier proposé est un répertoire.",
	err_cd_parent: "Erreur: Ne peux 'chdir' vers le dossier parent.",
	err_cd_execpath: "Erreur: Ne peux 'chdir' vers le dossier de l'exécutable.",
	comm_mut_excl: "Les commandes sont mutuellement exclusives.",
	mdp_gen_str: "Mot de passe généré:",
	usage: "[-f <motif>|-d <motif>|-n <desc>|-a <desc> <password>] [-l <longueur>] [fichier mdp]",
	options: OPTIONS_FR,
	ver: "version",
	ver_desc: "Gestion de mots de passe en CLI, version",
};

pub const LANG_ES: LangStrings = LangStrings {
	err_opt_l_1fois: "",
	err_opt_l_int: "",
	err_opt_l_bound: "",
	manque_args: "¡¡¡Faltan argumentos necesarios!!!\n-----",
	err_bad_arg: "",
	err_valid_args: "",
	err_cli: "",
	err_which: "",
	err_no_ssl: "",
	err_file_is_dir: "",
	err_cd_parent: "",
	err_cd_execpath: "",
	comm_mut_excl: "",
	mdp_gen_str: "",
	usage: "[-f|-d] [-riInv] <patrón de expresión regular> <reemplazo> [nombredirectorio ...]",
	options: OPTIONS_ES,
	ver: ": versión",
	ver_desc: ": Cambio de nombre múltiple basado en un patrón determinado, versión",
};

pub const LANG_EN: LangStrings = LangStrings {
	err_opt_l_1fois: "",
	err_opt_l_int: "",
	err_opt_l_bound: "",
	manque_args: "Missing necessary arguments!!\n-----",
	err_bad_arg: "",
	err_valid_args: "",
	err_cli: "",
	err_which: "",
	err_no_ssl: "",
	err_file_is_dir: "",
	err_cd_parent: "",
	err_cd_execpath: "",
	comm_mut_excl: "",
	mdp_gen_str: "",
	usage: "[-f|-d] [-riInv] <regex pattern> <replacement> [dirname ...]",
	options: OPTIONS_EN,
	ver: ": version",
	ver_desc: ": Multiple renaming based on a certain pattern, version",
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
