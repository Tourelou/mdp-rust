// actions.rs

use std::io::{self, Write};
use crate::locale::LangStrings;
use crate::{AppData, command_exist};
use crate::clipboard;

fn scan(ptrn: &String, list: &Vec<String>) -> Vec<usize> {
	// Retourne en vecteur d'index les lignes qui match le pattern.
	let pattern_lower = ptrn.to_lowercase();

	let mut x = 0;
	let mut y = 0;
	let mut selected_vs_full = Vec::new();

	for ligne in list {
		if ligne.to_lowercase().contains(&pattern_lower) {
			selected_vs_full.push(y);
			if let Some((avant, apres)) = ligne.split_once("∫∆∫") {
				x += 1;		// On incrémente x si nous avons une correspondance
				println!("{x:3}: {apres} ==> password: {avant}"); 
			}
		}
		y += 1;		// On incrémente y peut importe
	}
	selected_vs_full
}

fn get_index(max_len: usize, loc: &LangStrings) -> usize {
	// Demande à l'usager quelle ligne traiter.
	let mut input = String::new();

	// 1. Lire la saisie
	io::stdin()
		.read_line(&mut input)
		.expect(format!("{}", loc.err_keyboard).as_str());

	// 2. Tenter de convertir en nombre (usize est idéal pour les index)
	match input.trim().parse::<usize>() {
		Ok(index) => {
			if index > max_len { 
				println!("{}", loc.err_index_too_big.replace("{1}", index.to_string().as_str()));
				return 0;
			}
			else { return index; }
		}
		Err(_) => { return 0; }
	}
}

pub fn find(ptrn: &String, data: &AppData) {
	println!("----------------");
	let trouve = scan(&ptrn, &data.app_line_vec);
	if trouve.is_empty() {
		println!("\t------ {}", data.app_locale.no_match.replace("{1}", ptrn));
		println!("----------------");
	}
	else {
		if command_exist("pbcopy", &data.app_locale) {
			println!("----------------");
			print!("{}", data.app_locale.index_to_clip);
			let _ = io::stdout().flush();

			let index = get_index(trouve.len(), &data.app_locale);
			if index > 0 {
				if let Some((pw, _)) = trouve.get(index - 1)
					.and_then(|&i| data.app_line_vec[i].split_once("∫∆∫"))
				{
					clipboard::send_to_clipboard(pw);
				}
			}
		}
	}
}

pub fn del(ptrn: &String, data: &mut AppData) -> bool {
	println!("----------------");
	let trouve = scan(&ptrn, &data.app_line_vec);
	if trouve.is_empty() {
		println!("\t------ {}", data.app_locale.no_match.replace("{1}", ptrn));
		println!("----------------");
		return false;
	}
	else {
		println!("----------------");
		print!("{}", data.app_locale.index_to_del);
		let _ = io::stdout().flush();

		let index = get_index(trouve.len(), &data.app_locale);
		if index > 0 {		// Suppression
			if let Some(&index_a_supprimer) = trouve.get(index - 1) {
				if let Some((_, del_desc)) = data.app_line_vec
												.remove(index_a_supprimer)
												.split_once("∫∆∫") {
					println!("{}", data.app_locale.del_success.replace("{1}", del_desc));
					return true;
				}
				else { return false; }
			}
			else { return false; }
		}
		else { return false; }
	}
}