use riva::editor::Editor;

// const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> crossterm::Result<()> {
    let mut editor = Editor::new();
    while editor.run()? {}

    Ok(())
}
