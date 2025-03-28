/*
MIT License

Copyright (c) 2025 Christopher Sozio

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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