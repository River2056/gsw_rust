use dialoguer::{theme::ColorfulTheme, FuzzySelect, console::Term};

fn main() -> std::io::Result<()> {
    let items = vec!["items1", "items2", "items3"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    
    match selection {
        Some(index) => println!("User selected item: {}", items[index]),
        None => println!("User did not select anything")
    }

    Ok(())
}
