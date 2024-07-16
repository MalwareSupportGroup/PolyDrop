mod builder;
use builder::arguments;

fn main() {
    let args = arguments::getargs();
    let argsvec: Vec<&str> = args.split_whitespace().collect();
    let platform: String = argsvec[0].to_string(); // Platform
    let language: String = argsvec[1].to_string(); // Language
    let payload: String = argsvec[2].to_string(); // Payload
    let output: String = argsvec[3].to_string(); // Output file
    let allarg: String = argsvec[4].to_string(); // All value
    let lhostarg: String = argsvec[5].to_string(); // LHOST value
    let lportarg: String = argsvec[6].to_string(); // LPORT value
    let urlarg: String = argsvec[7].to_string(); // URL value
    builder::payload_builder(&platform, &language, &payload, &output, &allarg, &lhostarg, &lportarg, &urlarg);
}
