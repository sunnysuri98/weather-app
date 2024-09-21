mod logic;

use logic::{prompt, weather_info,clr};


fn main() {
    clr();

    println!(
        r#"
                       __  .__                                           __                  
__  _  __ ____ _____ _/  |_|  |__   ___________    _________.__. _______/  |_  ____   _____  
\ \/ \/ // __ \\__  \\   __\  |  \_/ __ \_  __ \  /  ___<   |  |/  ___/\   __\/ __ \ /     \ 
 \     /\  ___/ / __ \|  | |   Y  \  ___/|  | \/  \___ \ \___  |\___ \  |  | \  ___/|  Y Y  \
  \/\_/  \___  >____  /__| |___|  /\___  >__|    /____  >/ ____/____  > |__|  \___  >__|_|  /
             \/     \/          \/     \/             \/ \/         \/            \/      \/ 
"#
    );

    loop {
        println!("\nPress 'q' to quit or 'w' to get the weather");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        match input.trim().to_lowercase().as_str() {
            "w" => {
                let city = prompt("\nPlease enter the name of the city");
                weather_info(&city);
            }
            "q" => break println!("\nThank you for using our system. We hope it met your expectations"),
            _ => println!("Invalid input. Please press 'q' to quit or 'w' to get the weather."),
        }
    }
}

