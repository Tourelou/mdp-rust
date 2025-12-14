// openssl_cli.rs

mod parse;
mod locale;

use std::env;
// use std::io::{self, Write};
use std::process::ExitCode;

/// Déchiffre un fichier en utilisant l'exécutable OpenSSL via le CLI.
/// Le contenu déchiffré est retourné sous forme de vecteur d'octets.

fn decrypt_via_cli(encrypted_filepath: &str, password: &str,) -> Result<Vec<u8>, io::Error> {

	// Commande OpenSSL complète :
	// openssl enc -aes-256-cbc -d -md md5 -in <file> -pass pass:<password>
	let mut command = Command::new("openssl");

	command
		// La sous-commande de chiffrement/déchiffrement
		.arg("enc")
		// Algorithme de déchiffrement
		.arg("-aes-256-cbc")
		// Option de déchiffrement (-d)
		.arg("-d")
		// Algorithme de hachage pour la dérivation de clé (-md md5)
		.arg("-md").arg("md5")
		// Spécifie le fichier d'entrée
		.arg("-in").arg(encrypted_filepath)
		// Spécifie le mot de passe sur la ligne de commande (-pass pass:<password>)
		// C'est le moyen le plus simple d'injecter le mot de passe sans nécessiter d'interaction.
		.arg("-pass").arg(format!("pass:{}", password))
		// Capture la sortie standard (stdout) pour récupérer les données déchiffrées
		.stdout(Stdio::piped())
		// Ignore l'erreur standard (stderr) pour ne pas la mélanger au résultat
		.stderr(Stdio::null()); // Redirige stderr vers l'équivalent de /dev/null

	println!("Exécution de la commande OpenSSL...");

	// Exécuter la commande
	let output = command.output()?;

	// Vérifier si la commande a réussi
	if output.status.success() {
		// Retourner le contenu déchiffré capturé dans stdout
		Ok(output.stdout)
	} else {
		// Si OpenSSL échoue, retourner l'erreur avec le statut de sortie
		let stderr_message = String::from_utf8_lossy(&output.stderr);
		Err(io::Error::new(
			io::ErrorKind::Other,
			format!("OpenSSL a échoué. Code de sortie: {:?}. Message: {}", output.status.code(), stderr_message)
		))
	}
}
