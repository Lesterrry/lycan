use clap::App;
use clap::Arg;
extern crate rand;
use rand::Rng;
extern crate directories;
use directories::{UserDirs};
use std::fs::File;
use std::time::SystemTime;
use std::io::{Write, Read};
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct Token {
	timestamp: u64,
	raw: u64
}
impl Token{
	fn to_string(self) -> String{
		return self.timestamp.to_string() +  ":" + &self.raw.to_string()
	}
}

fn main() {
	let matches = App::new("lycan")
		.version("0.3.0")
		.about("2FA Tool")
		.author("Aydar N.")
		.arg(Arg::with_name("SEED")
			.help("Token to generate one-time password, must be equal for both signing and checking instances.")
			.required(false)
			.index(1))
		.arg(Arg::with_name("perform")
			.short("p")
			.long("prfrm")
			.help("Generate new one-time password token based on either passed input or saved data."))
		.arg(Arg::with_name("generate")
			.short("g")
			.long("gen")
			.help("Generate and save new seed token"))
		.arg(Arg::with_name("raw output")
			.short("r")
			.long("raw")
			.help("Show output in machine-recognizable format"))
		.get_matches();
	let raw_out = matches.is_present("raw output");
	if matches.is_present("generate"){
		let mut path = PathBuf::new();
		if let Some(user_dirs) = UserDirs::new() {
			let res = user_dirs.home_dir();
			path.push(res);
			path.push("lycan_token.ff")
		}
		let val = token_generate();
		let mut file = File::create(path).unwrap();
		write!(&mut file, "{}",val.clone().to_string()).unwrap();
		println!("{}{}", if !raw_out{"Generated token:\r\n"} else {""}, val.clone().raw);
	}
	else if matches.is_present("perform"){
		if matches.is_present("SEED"){
			let val = token_decode(matches.value_of("SEED").unwrap());
			 if !raw_out {
                                println!("Performing with token created {} days ago:", (get_now() - val.timestamp) / 86400);
                                println!("`{}`", token_perform(val));
                        }
                        else {println!("{}", token_perform(val))}
		}
		else{
			let mut path = PathBuf::new();
			if let Some(user_dirs) = UserDirs::new() {
				let res = user_dirs.home_dir();
				path.push(res);
				path.push("lycan_token.ff")
			}
			let mut file = File::open(path).expect("No saved tokens or passed inputs");
			let mut data: String = "".to_string();
			file.read_to_string(&mut data).expect("No saved tokens or passed inputs");
			let val = token_decode(&data);
			if !raw_out {
                                println!("Performing with token created {} days ago:", (get_now() - val.timestamp) / 86400);
                                println!("`{}`", token_perform(val));
                        }
                        else {println!("{}", token_perform(val))}
		}
	}
}
fn get_now() -> u64{
	let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
	let a: u64;
	match now {
		Ok(n) => a = n.as_secs(),
		Err(_) => panic!("SystemTime set wrong"),
	}
	return a;
}
fn token_generate() -> Token {
	let raw: u64 = rand::thread_rng().gen_range(1000000000000000000, 9223372036854775807);
	return Token{timestamp: get_now(), raw: raw};
}
fn token_decode(token: &str) -> Token{
	let collected: Vec<&str> = token.split(":").collect();
	let t = Token{timestamp: collected[0].parse::<u64>().unwrap(), raw: collected[1].parse::<u64>().unwrap()};
	return t;
}
fn token_perform(token: Token) -> u128{
	let now = get_now() / 10;
	let mut fingerprint = (now).to_string();
	let chopper = (token.raw / 10000000000000).to_string();
	for letter in chopper.chars(){
		fingerprint = fingerprint.replace(&letter.to_string(), "w");
	}
	let pusher: Vec<char> = ((token.raw % 10000000000000) / 1000000).to_string().chars().collect();
	fingerprint = fingerprint.replace("w", &collide(&pusher).to_string());
	let appender = (token.raw % 10000000000000) % 1000000;
	fingerprint += &((appender * token.timestamp) / 1000).to_string();
	return (fingerprint.parse::<u128>().unwrap() * ((now % 10) + 1) as u128) / (token.timestamp * 100000000) as u128 % (100000) as u128;
}
fn collide(input: &[char]) -> i64{
	let mut out = 0;
	for letter in input{
		out += letter.to_string().parse::<i64>().unwrap()
	}
	return out
}
