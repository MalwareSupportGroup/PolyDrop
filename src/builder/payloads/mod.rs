pub struct Payloads {
    tcltorture: String,
    pinkelephant: String,
    crystalmethod: String,
    juliachilds: String,
    gopherguts: String,
    dartwingduck: String,
    deeznuts: String,
    vendetta: String,
    noodles: String,
    buns: String,
    pythos: String,
    fsociety: String,
    denote: String,
}
impl Payloads {
    pub fn new(
        tcl_torture: &str,
        pink_elephant: &str,
        crystal_method: &str,
        julia_childs: &str,
        gopher_guts: &str,
        dartwing_duck: &str,
        deez_nuts: &str,
        v_endetta: &str,
        no_odles: &str,
        bun_s: &str,
        py_thos: &str,
        f_society: &str,
        d_note: &str,
    ) -> Payloads {
        Payloads {
            tcltorture: tcl_torture.to_string(),
            pinkelephant: pink_elephant.to_string(),
            crystalmethod: crystal_method.to_string(),
            juliachilds: julia_childs.to_string(),
            gopherguts: gopher_guts.to_string(),
            dartwingduck: dartwing_duck.to_string(),
            deeznuts: deez_nuts.to_string(),
            vendetta: v_endetta.to_string(),
            noodles: no_odles.to_string(),
            buns: bun_s.to_string(),
            pythos: py_thos.to_string(),
            fsociety: f_society.to_string(),
            denote: d_note.to_string(),
        }
    }
    pub fn tcl_payload(&self) -> String {
        format!("{}", self.tcltorture)
    }
    pub fn php_payload(&self) -> String {
        format!("{}", self.pinkelephant)
    }
    pub fn crystal_payload(&self) -> String {
        format!("{}", self.crystalmethod)
    }
    pub fn julia_payload(&self) -> String {
        format!("{}", self.juliachilds)
    }
    pub fn go_payload(&self) -> String {
        format!("{}", self.gopherguts)
    }
    pub fn dart_payload(&self) -> String {
        format!("{}", self.dartwingduck)
    }
    pub fn d_payload(&self) -> String {
        format!("{}", self.deeznuts)
    }
    pub fn v_payload(&self) -> String {
        format!("{}", self.vendetta)
    }
    pub fn node_payload(&self) -> String {
        format!("{}", self.noodles)
    }
    pub fn bun_payload(&self) -> String {
        format!("{}", self.buns)
    }
    pub fn python_payload(&self) -> String {
        format!("{}", self.pythos)
    }
    pub fn fsharp_payload(&self) -> String {
        format!("{}", self.fsociety)
    }
    pub fn deno_payload(&self) -> String {
        format!("{}", self.denote)
    }
}

pub struct Codex {
    tcl: String,
    php: String,
    crystal: String,
    julia: String,
    go: String,
    dart: String,
    d_lang: String,
    v_lang: String,
    node: String,
    bun: String,
    python: String,
    fsharp: String,
    deno: String,
}

impl Codex {
    pub fn new(
        tcl_shell: &str,
        php_shell: &str,
        crystal_shell: &str,
        julia_shell: &str,
        go_shell: &str,
        dart_shell: &str,
        d_shell: &str,
        v_shell: &str,
        node_shell: &str,
        bun_shell: &str,
        py_shell: &str,
        f_shell: &str,
        deno_shell: &str,
    ) -> Codex {
        Codex {
            tcl: tcl_shell.to_string(),
            php: php_shell.to_string(),
            crystal: crystal_shell.to_string(),
            julia: julia_shell.to_string(),
            go: go_shell.to_string(),
            dart: dart_shell.to_string(),
            d_lang: d_shell.to_string(),
            v_lang: v_shell.to_string(),
            node: node_shell.to_string(),
            bun: bun_shell.to_string(),
            python: py_shell.to_string(),
            fsharp: f_shell.to_string(),
            deno: deno_shell.to_string(),
        }
    }
    pub fn tcl_revshell(&self) -> String {
        format!("{}", self.tcl)
    }
    pub fn php_revshell(&self) -> String {
        format!("{}", self.php)
    }
    pub fn crystal_revshell(&self) -> String {
        format!("{}", self.crystal)
    }
    pub fn julia_revshell(&self) -> String {
        format!("{}", self.julia)
    }
    pub fn go_revshell(&self) -> String {
        format!("{}", self.go)
    }
    pub fn dart_revshell(&self) -> String {
        format!("{}", self.dart)
    }
    pub fn d_revshell(&self) -> String {
        format!("{}", self.d_lang)
    }
    pub fn v_revshell(&self) -> String {
        format!("{}", self.v_lang)
    }
    pub fn node_revshell(&self) -> String {
        format!("{}", self.node)
    }
    pub fn bun_revshell(&self) -> String {
        format!("{}", self.bun)
    }
    pub fn python_revshell(&self) -> String {
        format!("{}", self.python)
    }
    pub fn f_revshell(&self) -> String {
        format!("{}", self.fsharp)
    }
    pub fn deno_revshell(&self) -> String {
        format!("{}", self.deno)
    }
}
