use console::{
    measure_text_width, pad_str, strip_ansi_codes, style, truncate_str, Alignment, Key, Term,
};
use dialoguer::Input;
use std::io::Result as IOResult;

pub fn test() -> IOResult<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    let welcome_text = format!("{}", style("Welcome To console crate test!").cyan());
    let truncated_string = format!("{}", truncate_str(&welcome_text, 20, "..."));
    term.write_line(&truncated_string)?;
    term.write_line("Hello World")?;
    term.clear_last_lines(1)?;

    // test measure_text_width and strip_ansi_codes
    let text_width = measure_text_width(&welcome_text);
    let stripped_text_width = measure_text_width(&strip_ansi_codes(&welcome_text).to_string());
    term.write_line(&format!("{}/{}", stripped_text_width, text_width))?;
    term.write_line(&strip_ansi_codes(&welcome_text).to_string())?;

    // test pad_str
    term.write_line(&pad_str("Hejhej", 40, Alignment::Right, Some(".,.")).to_string())?;
    term.write_line(&pad_str("Hejhej", 4, Alignment::Right, Some(".")).to_string())?;

    // tes interact_on
    let input = Input::<String>::new()
        .with_prompt("Type something!")
        .interact_on(&term)?;

    term.write_line(&input)?;

    // Test read_line
    let line = term.read_line()?;
    term.write_line(&line)?;

    // Test read_key
    loop {
        let key = term.read_key()?;
        match key {
            Key::ArrowDown => term.write_str("Arrow Down")?,
            Key::Char(c) => term.write_str(&format!("{}", c))?,
            Key::Enter => break,
            _ => term.write_str("Something Different")?,
        }
    }

    // Cursor Movement
    term.move_cursor_up(3)?;

    Ok(())
}
