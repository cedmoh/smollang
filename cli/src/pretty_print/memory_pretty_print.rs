use colored::Colorize;

pub trait MemoryPrettyPrint {
    fn pretty_print(&self, colors_enabled: bool) -> String;
}

impl MemoryPrettyPrint for vm::Memory {
    fn pretty_print(&self, colors_enabled: bool) -> String {
        self.data
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
