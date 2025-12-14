// generator.rs

use std::hash::{Hasher, DefaultHasher};
use std::time::{SystemTime, UNIX_EPOCH};

// --- Constantes du jeu de caractères ---
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8]    = b"0123456789";
const SPECIALS: &[u8]  = b"!@#$?&_.~-";

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$?&_.~-";

// --- 1. Structure du générateur Xorshift* ---

/// Générateur de nombres pseudo-aléatoires (PRNG) Xorshift* 64 bits.
/// Il maintient un état interne (state) qui est mis à jour à chaque appel.
struct XorShiftStar {
	state: u64,
}

impl XorShiftStar {
	/// Initialise le générateur en utilisant l'heure système et l'adresse mémoire
	/// (pour une meilleure entropie initiale) pour obtenir la graine (seed).
	fn new() -> Self {
		let stack_variable = 0;
		// Bruit d'adresse (léger bruit non cryptographique)
		let address_noise = &stack_variable as *const i32 as u64;

		// Bruit de temps (nanosecondes depuis l'époque UNIX)
		let time_noise = match SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(n) => n.as_nanos() as u64,
			Err(_) => 1_u64,
		};

		// Combinaison pour la graine initiale
		let mut hasher = DefaultHasher::new();
		hasher.write_u64(address_noise);
		hasher.write_u64(time_noise);
		
		let mut state = hasher.finish();
		
		// Assurer que l'état n'est pas 0 (sinon le générateur se bloque)
		if state == 0 { state = 1; }

		Self { state }
	}

	/// Calcule et retourne le prochain nombre pseudo-aléatoire de 64 bits,
	/// tout en mettant à jour l'état interne.
	fn next_u64(&mut self) -> u64 {
		// Applique les étapes Xorshift
		self.state ^= self.state << 12;
		self.state ^= self.state >> 25;
		self.state ^= self.state << 27;
		
		// Applique l'étape multiplication (*)
		self.state.wrapping_mul(2685821657736338717)
	}

	/// Calcule un index aléatoire dans la plage [0, max-1].
	fn random_index(&mut self, max: usize) -> usize {
		(self.next_u64() % (max as u64)) as usize
	}
}

// --- 2. Fonctions d'aide (prenant le générateur en paramètre) ---

fn random_char_from(generator: &mut XorShiftStar, set: &[u8]) -> char {
	set[generator.random_index(set.len())] as char
}

// --- 3. Fonction principale ---

/// Génère un mot de passe de longueur `length`
/// contenant au moins une majuscule, une minuscule,
/// un chiffre et un caractère spécial.
pub fn gen_pass(length: usize) -> String {
	// Initialiser le générateur UNE SEULE FOIS pour toute la fonction
	let mut rng = XorShiftStar::new(); 

	let mut password: Vec<char> = Vec::with_capacity(length);

	// 1. Garantir au moins un de chaque catégorie
	password.push(random_char_from(&mut rng, UPPERCASE));
	password.push(random_char_from(&mut rng, LOWERCASE));
	password.push(random_char_from(&mut rng, DIGITS));
	password.push(random_char_from(&mut rng, SPECIALS));

	// 2. Compléter avec des caractères aléatoires (en utilisant le CHARSET complet)
	for _ in 4..length {
		password.push(random_char_from(&mut rng, CHARSET));
	}

	// 3. Mélanger (Fisher-Yates)
	// On utilise la méthode random_index du générateur pour obtenir de nouvelles valeurs aléatoires
	for i in (1..password.len()).rev() {
		let j = rng.random_index(i + 1);
		password.swap(i, j);
	}

	password.into_iter().collect()
}
