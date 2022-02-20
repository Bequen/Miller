type CmdHandler = fn(Vec<String>);

pub struct Cmd {
    cmd: String,
    args: Vec<String>,
    options: Vec<Cmd>,
    func: Option<CmdHandler>
}

impl Cmd {
    pub fn new(cmd: &str, func: Option<CmdHandler>) -> Self {
        Cmd {
            cmd: String::from(cmd),
            args: Vec::new(),
            options: Vec::new(),
            func: func
        }
    }

    pub fn add_arg(&mut self, arg:String) -> &Self {
        self.args.push(arg);

        return self;
    }

    pub fn add_option(&mut self, option: Cmd) -> &Self {
        self.options.push(option);

        return self;
    }
}

pub fn parse_cmd(cmd: &Cmd, str: String) {
    
}