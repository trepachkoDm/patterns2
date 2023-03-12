use patterns2::command::{AddTextCommand, RemoveTextCommand, TextEditor};
fn main() {
    let mut editor = TextEditor::new();
    let text = "Hello, World!".to_owned();

    editor.add_command(Box::new(AddTextCommand {
        text: " Welcome!".to_owned(),
        target: text.clone(),
    }));
    editor.add_command(Box::new(RemoveTextCommand {
        text: " World".to_owned(),
        target: text.clone(),
    }));

    editor.execute_commands();
    println!("{}", text);

    editor.add_command(Box::new(AddTextCommand {
        text: " Rust!".to_owned(),
        target: text.clone(),
    }));
    editor.add_command(Box::new(RemoveTextCommand {
        text: " World".to_owned(),
        target: text.clone(),
    }));

    editor.execute_commands();
    println!("{}", text);
}
