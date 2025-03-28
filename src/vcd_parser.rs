pub struct VCD {
    pub file: String,
    pub date: String,
    pub timescale: String,
    pub version: String,
}

impl VCD {
    pub fn new(file: &String) -> VCD {
        let v = VCD {
            file: file.clone(),
            date: String::new(),
            timescale: String::new(),
            version: String::new(),
        };

        return v;
    }

    pub fn print_members(&self) {
        println!("{}\n\tDate: {}\n\tTimescale: {}\n\tVersion: {}\n", self.file, self.date, self.timescale, self.version);
    }
}

#[derive (Debug)]
pub struct ValueChange {
    pub time: u64,
    pub value: String,
}

#[derive (Debug)]
pub struct Var {
    pub scope: String,
    pub scope_type: String,
    pub var_type: String,
    pub size: u8,
    pub identifier: String,
    pub reference: String,
    pub changes: Vec<ValueChange>,
}

impl Var {
    pub fn new(scope: String, scope_type: String, var_type: String, size: u8, identifier: String, reference: String) -> Var {
        let change: Vec<ValueChange> = Vec::new();

        let v = Var {
            scope: scope,
            scope_type: scope_type,
            var_type: var_type,
            size: size,
            identifier: identifier,
            reference: reference,
            changes: change,
        };

        return v
    }
}