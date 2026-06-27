use colored::Colorize;
use vm::{Memory, ValueStack};

pub trait MemoryPrettyPrint {
    fn pretty_print(&self, colors_enabled: bool) -> String;
}

impl MemoryPrettyPrint for Memory {
    fn pretty_print(&self, colors_enabled: bool) -> String {
        let dumped = self.clone().dump();

        dumped
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                let address = format!("{:04X}", index);
                let value = match cell {
                    Some(object) => format!("{:?}", object),
                    None => "None".to_string(),
                };

                if colors_enabled {
                    format!("{}  {}", address.blue(), value.green())
                } else {
                    format!("{}  {}", address, value)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl MemoryPrettyPrint for ValueStack {
    fn pretty_print(&self, colors_enabled: bool) -> String {
        let dumped = self.clone().dump();

        if dumped.is_empty() {
            return if colors_enabled {
                "<empty>".white().italic().to_string()
            } else {
                "<empty>".to_string()
            };
        }

        dumped
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let address = format!("{:04X}", index);
                let value_str = format!("{:?}", value);

                if colors_enabled {
                    format!("{}  {}", address.blue(), value_str.green())
                } else {
                    format!("{}  {}", address, value_str)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
