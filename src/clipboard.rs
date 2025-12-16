use std::process::{Command, Stdio};
use std::io::Write;

/// Tente de copier la chaîne de caractères sur le presse-papiers macOS en utilisant 'pbcopy'.
/// Retourne 'true' si l'opération a réussi, 'false' sinon.
fn send_to_clipboard(text: &str) -> bool {
	// 1. Prépare la commande, en demandant un pipe pour l'entrée standard
	let mut child = match Command::new("pbcopy").stdin(Stdio::piped()).spawn()
	{
		Ok(c) => c,
		Err(_) => return false, // Échec du lancement de pbcopy
	};
	// 2. Tente d'écrire le texte dans le pipe
	let success = if let Some(mut stdin) = child.stdin.take() {
		// Écrit le texte dans le pipe
		match stdin.write_all(text.as_bytes()) {
			Ok(_) => true,
			Err(_) => false,
		}
		// Le pipe est fermé ici lorsque 'stdin' sort du scope
	}
	else { false };		// Impossible d'obtenir le pipe d'entrée

	// 3. Attendre la fin du processus et vérifier le succès
	// Même si l'écriture a échoué, nous devons attendre le processus.
	let status = match child.wait() {
		Ok(s) => s,
		Err(_) => return false, // Échec de l'attente du processus
	};
	
	// 4. Retourne vrai seulement si l'écriture a réussi ET le processus s'est terminé avec succès
	success && status.success()
}

fn main() {
	let my_string = "Tentative de copie via pbcopy";

	if send_to_clipboard(my_string) { println!("✅ {my_string} ==> Clipboard."); }
	else { eprintln!("❌ Échec de la copie."); }
}
