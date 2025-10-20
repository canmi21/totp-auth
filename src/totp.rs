/* src/totp.rs */

use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

/// Generate a single 6-digit TOTP token.
///
/// # Arguments
///
/// * `seed` - The shared secret seed string.
/// * `time` - The current UNIX timestamp in seconds.
/// * `window` - The time step window size in seconds.
///
/// # Returns
///
/// A 6-digit unsigned integer token.
///
/// # Example
///
/// ```
/// let token = generate_token("secret", 1700000000, 30);
/// println!("{:06}", token);
/// ```
fn generate_token(seed: &str, time: u64, window: u64) -> u32 {
	let counter = time / window;
	let mut mac = HmacSha1::new_from_slice(seed.as_bytes()).unwrap();
	mac.update(&counter.to_be_bytes());
	let hash = mac.finalize().into_bytes();
	let offset = (hash[hash.len() - 1] & 0x0f) as usize;
	let code = ((u32::from(hash[offset]) & 0x7f) << 24)
		| ((u32::from(hash[offset + 1]) & 0xff) << 16)
		| ((u32::from(hash[offset + 2]) & 0xff) << 8)
		| (u32::from(hash[offset + 3]) & 0xff);
	code % 1_000_000
}

/// Generate a combined TOTP token from six different seeds.
///
/// # Arguments
///
/// * `seeds` - An array of six secret seeds.
/// * `time` - The current UNIX timestamp in seconds.
/// * `window` - The time step window size in seconds.
///
/// # Returns
///
/// A string composed of six tokens joined with hyphens (`-`).
///
/// # Example
///
/// ```
/// let seeds = ["a", "b", "c", "d", "e", "f"];
/// let token = generate_combined_token(seeds, 1700000000, 30);
/// println!("{}", token); // "123456-654321-..."
/// ```
pub fn generate_combined_token(seeds: [&str; 6], time: u64, window: u64) -> String {
	let tokens: Vec<String> = seeds
		.iter()
		.map(|s| format!("{:06}", generate_token(s, time, window)))
		.collect();
	tokens.join("-")
}

/// Verify a combined TOTP token across multiple allowed time windows.
///
/// # Arguments
///
/// * `seeds` - An array of six secret seeds used to generate the token.
/// * `time` - The current UNIX timestamp in seconds.
/// * `token` - The combined token string to verify.
/// * `window` - The time step window size in seconds.
/// * `allowed_windows` - The number of time windows (before and after) to allow for drift.
/// * `unit` - The time unit (e.g., `"s"` for seconds; currently unused placeholder).
///
/// # Returns
///
/// `true` if the token is valid within the allowed window range, otherwise `false`.
///
/// # Example
///
/// ```
/// let seeds = ["a", "b", "c", "d", "e", "f"];
/// let now = current_unix_time();
/// let token = generate_combined_token(seeds, now, 30);
/// assert!(verify_combined_token(seeds, now, &token, 30, 1, "s"));
/// ```
pub fn verify_combined_token(
	seeds: [&str; 6],
	time: u64,
	token: &str,
	window: u64,
	allowed_windows: u32,
	unit: &str,
) -> bool {
	let delta = if unit == "s" { 1 } else { 1 };
	let steps = match allowed_windows {
		0 | 1 => vec![0],
		n => {
			let mut all = vec![0i64];
			for i in 1..n as i64 {
				all.push(i);
				all.push(-i);
			}
			all
		}
	};

	for step in steps {
		let t = if step == 0 {
			time
		} else {
			time.wrapping_add_signed(step * (window * delta) as i64)
		};
		let gen_token = generate_combined_token(seeds, t, window);
		if gen_token == token {
			return true;
		}
	}
	false
}

/// Get the current UNIX timestamp in seconds.
///
/// # Returns
///
/// The number of seconds since the UNIX epoch (January 1, 1970).
///
/// # Example
///
/// ```
/// let now = current_unix_time();
/// println!("Current timestamp: {}", now);
/// ```
pub fn current_unix_time() -> u64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs()
}
