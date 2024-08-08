use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref WIN_DEFAULT_URLS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("tcl", "https://storage.googleapis.com/google-code-archive-downloads/v2/code.google.com/tclkit/tclkitsh-8.5.9-win32-x86_64.zip");
        m.insert(
            "php",
            "https://windows.php.net/downloads/releases/php-8.3.6-nts-Win32-vs16-x64.zip",
        );
        m.insert("crystal", "https://github.com/crystal-lang/crystal/releases/download/1.12.1/crystal-1.12.1-windows-x86_64-msvc-unsupported.zip");
        m.insert(
            "julia",
            "https://julialang-s3.julialang.org/bin/winnt/x64/1.10/julia-1.10.2-win64.zip",
        );
        m.insert("golang", "https://go.dev/dl/go1.22.2.windows-amd64.zip");
        m.insert("dart", "https://storage.googleapis.com/dart-archive/channels/stable/release/3.3.4/sdk/dartsdk-windows-x64-release.zip");
        m.insert(
            "dlang",
            "https://downloads.dlang.org/releases/2.x/2.108.0/dmd.2.108.0.windows.zip",
        );
        m.insert(
            "vlang",
            "https://github.com/vlang/v/releases/latest/download/v_windows.zip",
        );
        m.insert(
            "nodejs",
            "https://nodejs.org/dist/v20.12.2/node-v20.12.2-win-x64.zip",
        );
        m.insert(
            "bun",
            "https://github.com/oven-sh/bun/releases/download/bun-v1.1.18/bun-windows-x64.zip",
        );
        m.insert(
            "python",
            "https://www.python.org/ftp/python/3.12.3/python-3.12.3-embed-amd64.zip",
        );
        m.insert("fsharp", "https://download.visualstudio.microsoft.com/download/pr/bf435b42-3f28-45db-a666-6e95c4faefe7/23e0b703124347b51f53faf64c829287/dotnet-sdk-8.0.204-win-x64.zip");
        m.insert("deno", "https://github.com/denoland/deno/releases/download/v1.45.1/deno-x86_64-pc-windows-msvc.zip");
        m
    };
}

lazy_static! {
    pub static ref LIN_DEFAULT_URLS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("tcl", "https://storage.googleapis.com/google-code-archive-downloads/v2/code.google.com/tclkit/tclkitsh-8.5.9-linux-ix86.gz");
        m.insert(
            "php",
            "https://dl.static-php.dev/static-php-cli/common/php-8.3.6-cli-linux-x86_64.tar.gz",
        );
        m.insert("crystal", "https://github.com/crystal-lang/crystal/releases/download/1.12.1/crystal-1.12.1-1-linux-x86_64.tar.gz");
        m.insert("julia", "https://julialang-s3.julialang.org/bin/linux/x64/1.10/julia-1.10.3-linux-x86_64.tar.gz");
        m.insert("golang", "https://go.dev/dl/go1.22.3.linux-amd64.tar.gz");
        m.insert("dart", "https://storage.googleapis.com/dart-archive/channels/stable/release/3.3.4/sdk/dartsdk-linux-x64-release.zip");
        m.insert(
            "dlang",
            "https://downloads.dlang.org/releases/2.x/2.108.1/dmd.2.108.1.linux.zip",
        );
        m.insert(
            "vlang",
            "https://github.com/vlang/v/releases/download/weekly.2024.19/v_linux.zip",
        );
        m.insert(
            "nodejs",
            "https://nodejs.org/dist/v20.12.2/node-v20.12.2-linux-x64.tar.gz",
        );
        m.insert(
            "bun",
            "https://github.com/oven-sh/bun/releases/download/bun-v1.1.18/bun-linux-x64.zip",
        );
        m.insert("python", "");
        m.insert("fsharp", "https://download.visualstudio.microsoft.com/download/pr/0a1b3cbd-b4af-4d0d-9ed7-0054f0e200b4/4bcc533c66379caaa91770236667aacb/dotnet-sdk-8.0.204-linux-x64.tar.gz");
        m.insert("deno", "https://github.com/denoland/deno/releases/download/v1.45.1/deno-x86_64-unknown-linux-gnu.zip");
        m
    };
}

lazy_static! {
    pub static ref MAC_DEFAULT_URLS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("tcl", "https://tclkits.rkeene.org/fossil/raw/tclkit-8.6.3-macosx10.5-ix86+x86_64?name=1b4a7ae47ebab6ea9e0e16af4d8714c8b4aa0ce2");
        m.insert(
            "php",
            "https://dl.static-php.dev/static-php-cli/common/php-8.3.6-cli-macos-x86_64.tar.gz",
        );
        m.insert("crystal", "https://github.com/crystal-lang/crystal/releases/download/1.12.1/crystal-1.12.1-1-darwin-universal.tar.gz");
        m.insert(
            "julia",
            "https://julialang-s3.julialang.org/bin/mac/x64/1.10/julia-1.10.3-mac64.tar.gz",
        );
        m.insert("golang", "https://go.dev/dl/go1.22.3.darwin-arm64.tar.gz");
        m.insert("dart", "https://storage.googleapis.com/dart-archive/channels/stable/release/3.3.4/sdk/dartsdk-macos-x64-release.zip");
        m.insert(
            "dlang",
            "https://downloads.dlang.org/releases/2.x/2.108.1/dmd.2.108.1.osx.zip",
        );
        m.insert(
            "vlang",
            "https://github.com/vlang/v/releases/download/weekly.2024.19/v_macos_x86_64.zip",
        );
        m.insert(
            "nodejs",
            "https://nodejs.org/dist/v20.12.2/node-v20.12.2-darwin-x64.tar.gz",
        );
        m.insert(
            "bun",
            "https://github.com/oven-sh/bun/releases/download/bun-v1.1.18/bun-darwin-x64.zip",
        );
        m.insert("python", "");
        m.insert("fsharp", "https://download.visualstudio.microsoft.com/download/pr/9548c95b-8495-4b69-b6f0-1fdebdbbf9ff/30827786409718c5a9604711661da3b5/dotnet-sdk-8.0.204-osx-x64.tar.gz");
        m.insert("deno", "https://github.com/denoland/deno/releases/download/v1.45.1/deno-x86_64-apple-darwin.zip");
        m
    };
}

lazy_static! {
    pub static ref OS_TO_DOWNLOAD_HASHMAP: HashMap<&'static str, &'static HashMap<&'static str, &'static str>> = {
        let mut m = HashMap::new();
        m.insert("windows", &*WIN_DEFAULT_URLS);
        m.insert("linux", &*LIN_DEFAULT_URLS);
        m.insert("mac", &*MAC_DEFAULT_URLS);
        m
    };
}
