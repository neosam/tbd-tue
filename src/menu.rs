use std::io;
use std::io::Write;
use std::str::FromStr;

pub struct Menu<T> {
    pub title_txt: String,
    pub back_txt: String,
    pub entries: Vec<(String, MenuEntry<T>)>,
    pub auto_back: bool
}

pub fn m<T>(title: &str, back: &str,
         entries: Vec<(&str, MenuEntry<T>)>) -> Menu<T> {
    let mut insert_entries: Vec<(String, MenuEntry<T>)> = Vec::new();
    for (id, menu_entry) in entries {
        insert_entries.push((id.to_string(), menu_entry));
    }
    Menu {
        title_txt: title.to_string(),
        back_txt: back.to_string(),
        entries: insert_entries,
        auto_back: true
    }
}
pub fn mm<T>(title: &str, back: &str,
          entries: Vec<(&str, MenuEntry<T>)>) -> Menu<T> {
    let mut insert_entries: Vec<(String, MenuEntry<T>)> = Vec::new();
    for (id, menu_entry) in entries {
        insert_entries.push((id.to_string(), menu_entry));
    }
    Menu {
        title_txt: title.to_string(),
        back_txt: back.to_string(),
        entries: insert_entries,
        auto_back: false
    }
}

pub enum MenuEntry<T> {
    SubMenu(Menu<T>),
    Function(fn (&mut T) -> bool)
}                

pub fn sub_menu<T>(menu: Menu<T>) -> MenuEntry<T> {
    MenuEntry::SubMenu(menu)
}

pub fn m_call<T>(func: fn (&mut T) -> bool) -> MenuEntry<T> {
    MenuEntry::Function(func)
}

pub fn flush_str(text: &str) {
    io::stdout().write(text.as_bytes())
        .expect("Could not write to stdout");
    io::stdout().flush()
        .expect("Could not flush stdout");
}

pub fn read_string(prompt: &str) -> String {
    let mut input = String::new();
    flush_str(prompt);
    io::stdin().read_line(&mut input)
        .expect("Could not read from stdin");
    input.trim().to_string()
}

pub fn read_parse<T: FromStr + Default>(prompt: &str) -> T {
    let mut result: T = T::default();
    loop {
        let mut input = String::new();
        flush_str(prompt);
        io::stdin().read_line(&mut input)
            .expect("Could not read line");
        match input.trim().parse() {
            Ok(num) => {
                result = num;
                break
            },
            Err(err) => {
                println!("Could not parse: {}", input.trim());
            }
        }
    };
    result
}

pub fn read_usize(prompt: &str) -> usize{
    let mut result: (usize, bool) = (0, false);
    while result.1 == false {
        let mut input = String::new();
        flush_str(prompt);
        io::stdin().read_line(&mut input)
            .expect("Could not read line");
        result = match input.trim().parse() {
            Ok(num) => (num, true),
            Err(err) => {
                println!("Could not parse: {}", err);
                (0, false)

            }
        }
    }
    result.0
}


impl<T> Menu<T> {
    pub fn print(&self) {
        println!("{}", self.title_txt);
        for i in 0..(self.entries.len()) {
            println!("{} - {}", i + 1, self.entries[i].0);
        }
        println!("0 - {}", self.back_txt);
    }

    pub fn run(&self, item: &mut T) {
        let mut done = false;
        while !done {
            println!("");
            self.print();
            let choose: usize = read_usize("> ");
            if choose > self.entries.len() {
                continue
            } else if choose == 0 {
                break
            } else {
                done = self.run_entry(choose, item)
            }
        }
    }

    pub fn run_entry(&self, i: usize, item: &mut T) -> bool {
        match self.entries[i - 1].1 {
            MenuEntry::SubMenu(ref menu) => {menu.run(item); self.auto_back},
            MenuEntry::Function(func) => func(item)
        }
    }
}
