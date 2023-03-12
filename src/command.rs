pub trait Command {
    fn execute(&mut self);
}

pub struct AddTextCommand {
    pub text: String,
    pub target: String,
}

impl Command for AddTextCommand {
    fn execute(&mut self) {
        self.target.push_str(&self.text);
    }
}

pub struct RemoveTextCommand {
    pub text: String,
    pub target: String,
}

impl Command for RemoveTextCommand {
    fn execute(&mut self) {
        self.target = self.target.replace(&self.text, "");
    }
}

pub struct TextEditor {
    pub commands: Vec<Box<dyn Command>>,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    pub fn execute_commands(&mut self) {
        for command in &mut self.commands {
            command.execute();
        }
        self.commands.clear();
    }
}
