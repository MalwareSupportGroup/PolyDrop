use clap::{arg, App, Arg};
struct ArgOptions {
    output: String,
    os: String,
    lang: String,
    payload: String,
    all: bool,
    lhost: String,
    lport: String,
    url: String,
    ldl: String,
}

impl ArgOptions {
    fn new(
        otpt: &str,
        ops: &str,
        lng: &str,
        pl: &str,
        todos: bool,
        lhst: &str,
        lprt: &str,
        rl: &str,
        ldl: &str,
    ) -> ArgOptions {
        ArgOptions {
            output: otpt.to_string(),
            os: ops.to_string(),
            lang: lng.to_string(),
            payload: pl.to_string(),
            all: todos,
            lhost: lhst.to_string(),
            lport: lprt.to_string(),
            url: rl.to_string(),
            ldl: ldl.to_string(),
        }
    }
}

pub fn getargs() -> String {
    let args = App::new("PolyDrop")
	.version("0.1.0")
	.about("Description: BYOSI (Bring-Your-Own-Script-Interpreter) Rapid Payload Deployment")
	.author("Author: The Malware Support Group")
    .usage("./polydrop -x <OS-Option> -s <Language> -t <Payload Type> -o <Output filename> -l/--lhost <IP address/Hostname> -p/--lport <Port Number> -u/--url <Remote URL> -z/--lang-download <LanguageToolChain>")
	.args(&[
		Arg::new("OS-Option")
			.short('x')
            .takes_value(true)
            .help("Operating System: windows, linux, macos"),
        Arg::new("Script-Language")
            .short('s')
            .takes_value(true)
            .required(false)
            .help("Script Language: tcl, php, crystal, julia, golang, dart, dlang, vlang, nodejs, bun, python, fsharp, deno"),
        Arg::new("Payload-Type")
			.short('t')
            .takes_value(true)
            .help("Payload type: pwsh, sh"),
        Arg::new("Output")
            .short('o')
            .takes_value(true)
            .help("Output filename: <anything goes>"),
        Arg::new("All")
            .long("--all")
            .required(false)
            .takes_value(false)
            .help("All payloads written to one payload"),
        Arg::new("LHOST")
            .short('l')
            .long("--lhost")
            .required(true)
            .takes_value(true)
            .help("Listener Host: <IP address/Hostname>"),
        Arg::new("LPORT")
            .short('p')
            .long("--lport")
            .required(true)
            .takes_value(true)
            .help("Listener Port: <port number>"),
        Arg::new("URL")
            .short('u')
            .long("--url")
            .required(true)
            .takes_value(true)
            .help("Remote URL for Payload Download: <Full webroot path from trusted server. E.g: https://raw.githubusercontent.com/your_account/repo_name/main/>"),
        Arg::new("LANG_DOWNLOAD")
            .short('z')
            .long("--lang-download")
            .required(false)
            .takes_value(true)
            .help("Optional argument for the specific toolchain to download.")
	]).get_matches();

    let allval;
    if args.is_present("All") {
        let (p, l, t, o, a, lh, lp, url) = (
            args.value_of("OS-Option").unwrap(),
            "empty",
            args.value_of("Payload-Type").unwrap(),
            args.value_of("Output").unwrap(),
            args.values_of("All").is_some(),
            args.value_of("LHOST").unwrap(),
            args.value_of("LPORT").unwrap(),
            args.value_of("URL").unwrap(),
        );
        let op = ArgOptions::new(o, p, l, t, a, lh, lp, url, "");
        //println!("PolyDrop");
        //println!("- BYOSI (Bring-Your-Own-Script-Interpreter) Rapid Payload Deployment");
        //println!(
        //    "OS: {}\nPayload Type: {}\nOutput: {}\n",
        //    op.os, op.payload, op.output
        //);
        //println!(
        //    "LHOST: {}\nLPORT: {}\nURL: {}\n",
        //    op.lhost, op.lport, op.url
        //);
        //println!("All: {}\n", op.all);
        allval = "true";
        return format!(
            "{} {} {} {} {} {} {} {}",
            op.os, op.lang, op.payload, op.output, allval, op.lhost, op.lport, op.url
        );
    } else {
        let (p, l, t, o, a, lh, lp, url, ldl) = (
            args.value_of("OS-Option").unwrap(),
            args.value_of("Script-Language").unwrap(),
            args.value_of("Payload-Type").unwrap(),
            args.value_of("Output").unwrap(),
            args.values_of("All").is_some(),
            args.value_of("LHOST").unwrap(),
            args.value_of("LPORT").unwrap(),
            args.value_of("URL").unwrap(),
            args.value_of("LANG_DOWNLOAD").unwrap_or(""),
        );
        let op = ArgOptions::new(o, p, l, t, a, lh, lp, url, ldl);
        //println!("PolyDrop");
        //println!("- BYOSI (Bring-Your-Own-Script-Interpreter) Rapid Payload Deployment");
        //println!(
        //    "OS: {}\nScript Language: {}\nPayload Type: {}\nOutput: {}\nLanguage Toolchain: {}\n",
        //    op.os, op.lang, op.payload, op.output, op.ldl
        //);
        //println!(
        //    "LHOST: {}\nLPORT: {}\nURL: {}\n",
        //    op.lhost, op.lport, op.url
        //);
        allval = "false";
        return format!(
            "{} {} {} {} {} {} {} {} {}",
            op.os, op.lang, op.payload, op.output, allval, op.lhost, op.lport, op.url, op.ldl
        );
    }
}
