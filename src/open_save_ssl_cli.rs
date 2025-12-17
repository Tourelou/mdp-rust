// open-save_ssl_cli.rs

use std::process::{Command, Stdio};
use std::io::{Write, Error, ErrorKind};

/// Déchiffre un fichier et retourne son contenu ligne par ligne.
pub fn decrypt_via_cli(encrypted_filepath: &str, password: &str) -> Result<Vec<String>, Error> {
	let output = Command::new("openssl")
.arg("enc")
		.arg("-aes-256-cbc")
		.arg("-d")
		.arg("-md").arg("md5")
		.arg("-in").arg(encrypted_filepath)
		.arg("-pass").arg(format!("pass:{}", password))
		.stdout(Stdio::piped())
		.stderr(Stdio::null()) // Silence total sur les erreurs
		.output()?;

	if output.status.success() {
		// Conversion des bytes en String
		let content = String::from_utf8(output.stdout)
			.map_err(|e| Error::new(ErrorKind::InvalidData, format!("Format UTF-8 invalide : {}", e)))?;
		
		// Découpage par ligne
		let lines = content.lines().map(|s| s.to_string()).collect();
		Ok(lines)
	} else {
		Err(Error::new(ErrorKind::Other, "Échec du déchiffrement OpenSSL"))
	}
}

/// Chiffre un Vec<String> et sauvegarde le résultat dans un fichier.
pub fn encrypt_via_cli(output_file: &str, data: &Vec<String>, password: &str) -> Result<(), Error> {
	// Préparation des données : on joint les lignes avec un saut de ligne
	let input_text = data.join("\n");

	let mut child = Command::new("openssl")
.arg("enc")
		.arg("-aes-256-cbc")
		.arg("-salt")
		.arg("-md").arg("md5")
		.arg("-out").arg(output_file)
		.arg("-pass").arg(format!("pass:{}", password))
		.stdin(Stdio::piped())
		.stderr(Stdio::null()) // Silence total sur les erreurs
		.spawn()?;

	// On récupère le handle vers le stdin de l'enfant
	if let Some(mut stdin) = child.stdin.take() {
		stdin.write_all(input_text.as_bytes())?;
	}

	let status = child.wait()?;

	if status.success() {
		Ok(())
	} else {
		Err(Error::new(ErrorKind::Other, "Échec du chiffrement OpenSSL"))
	}
}