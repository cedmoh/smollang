use colored::*;

use bytecode::{Assembly, Constant, Instruction};

pub trait AssemblyPrettyPrint {
    fn pretty_print(&self, colors_enabled: bool) -> String;
}

impl AssemblyPrettyPrint for Assembly {
    fn pretty_print(&self, colors_enabled: bool) -> String {
        let mut output = String::new();

        for (instruction_address, instruction) in
            Into::<Vec<Instruction>>::into(self.instructions.clone())
                .iter()
                .enumerate()
        {
            let instruction_str = instruction.pretty_print(colors_enabled);

            if let Some(label) =
                self.labels.get_label(instruction_address.into())
            {
                if colors_enabled {
                    output.push_str(&format!("{}:\n", label.name.red()));
                } else {
                    output.push_str(&format!("{}:\n", label.name));
                }
            }

            if colors_enabled {
                output.push_str(&format!(
                    "{}  {}\n",
                    format!("{:04}", instruction_address).blue(),
                    instruction_str
                ));
            } else {
                output.push_str(&format!(
                    "{:04}  {}\n",
                    instruction_address, instruction_str
                ));
            }
        }

        // Divider
        output.push_str(&format!("----\n",));

        for (i, constant) in self.constants.iter().enumerate() {
            let constant_str = constant.pretty_print(colors_enabled);

            if colors_enabled {
                output.push_str(&format!(
                    "{}  {}\n",
                    format!("{:04}", i).bright_blue(),
                    constant_str
                ));
            } else {
                output.push_str(&format!("{:04}  {}\n", i, constant_str));
            }
        }

        output
    }
}

impl AssemblyPrettyPrint for Instruction {
    fn pretty_print(&self, colors_enabled: bool) -> String {
        let name = self.name();
        let maybe_param = self.parameter_to_string();

        if colors_enabled {
            let name = name.red();

            match maybe_param {
                Some(param) => {
                    format!("{:<5}  {}", name, param.to_string().yellow())
                }
                None => name.to_string(),
            }
        } else {
            match maybe_param {
                Some(param) => format!("{:<5}  {}", name, param),
                None => name.to_string(),
            }
        }
    }
}

impl AssemblyPrettyPrint for Constant {
    fn pretty_print(&self, colors_enabled: bool) -> String {
        if colors_enabled {
            match self {
                Constant::Nil => {
                    format!("{}", "nil".yellow())
                }
                Constant::Int(_) => {
                    format!("{}", self.to_string().yellow())
                }
                Constant::Float(_) => {
                    format!("{}", self.to_string().red())
                }
                Constant::Boolean(_) => {
                    format!("{}", self.to_string().purple())
                }
                Constant::String(_) => {
                    format!("{}", self.to_string().green())
                }
            }
        } else {
            self.to_string()
        }
    }
}
