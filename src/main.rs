use game::combat::{Named, enter_combat};
use game::entities::{Player, Mob};
mod menu;

use menu::{run_menu, MenuChoice};
fn main() -> std::io::Result<()> {

    loop {
        match run_menu()? {
            MenuChoice::Fight => {
                let mut player = Player {
                    health: 100,
                    attack: 12,
                    gold: 0,
                    name: "Drew".into(),
                };

                let mut mobs = Mob::spawn_mobs(2);
                println!("Welcome, {}!", player.name);

                while let Some(mut mob) = mobs.pop() {
                    enter_combat(&mut player, &mut mob);
                }

                println!("{:?}", player);
                println!("\nPress Enter to return to menu...");
                let _ = std::io::stdin().read_line(&mut String::new());
            }
            MenuChoice::Quit => break,
        }
    }

    Ok(())
}






