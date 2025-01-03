use std::io::{self, Write};

fn main() {
pokedex();
}

fn pokedex(){
       // Prompt the user for input
       print!("Please enter a pokemon's name: ");

       // Make sure the prompt is printed to the screen immediately
       io::stdout().flush().unwrap();
   
       // Declare a mutable variable to hold the user input
       let mut name = String::new();
   
       // Read the user input into the 'name' variable
       io::stdin().read_line(&mut name).unwrap();
   
       // Trim the input to remove the newline character at the end
       let name = name.trim();

    match name {
        "Bulbasaur" => println!("
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠉⢳⠴⢲⠂⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⠤⠤⠤⠤⠤⠤⠤⠤⠤⠖⠊⠀⣠⠎⠀⡞⢹⠏⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡴⠊⠁⠀⠀⠀⠀⠀⢀⡠⠤⠄⠀⠀⠀⠁⠀⠀⢀⠀⢸⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣠⠤⠤⠄⣀⠀⠀⠀⠀⢀⣌⠀⠀⠀⠀⠀⢀⣠⣆⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠘⡄⠀⠀⠀⠀
⠀⠀⠀⠀⡴⠁⠀⠀⠐⠛⠉⠁⠀⠀⣉⠉⠉⠉⠑⠒⠉⠁⠀⠀⢸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢧⠀⠱⡀⠀⠀⠀
⠀⠀⠀⢰⣥⠆⠀⠀⠀⣠⣴⣶⣿⣿⣿⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⢇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⡆⠀⠑⡄⠀⠀
⠀⠀⢀⡜⠁⠀⠀⢀⠀⠻⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠀⠀⠸⡀⠀
⠀⢀⣮⢖⣧⢠⠀⣿⠇⠀⠀⠁⠀⠀⠀⠠⠀⢀⣠⣴⣤⡀⠀⠀⠀⠈⡗⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⢱⠀
⠀⣼⠃⣼⣿⠘⠀⠀⠀⢠⣶⣿⡆⠀⠀⠁⣠⠊⣸⣿⣿⣿⡄⠀⠀⠀⡇⠀⢑⣄⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠸⡆
⠀⣿⢰⣿⣿⠀⠀⠀⠀⠙⠻⠿⠁⠀⠀⠠⠁⠀⣿⣿⣿⣿⡇⠀⠀⠀⠇⠀⢻⣿⣷⣦⣀⡀⣀⠠⠋⠀⠀⠀⢀⡇
⠈⠉⠺⠿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⢿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠈⢿⣿⣿⣿⣿⢦⡀⠀⠀⠀⠀⡸⠀
⠘⣟⠦⢀⠀⠀⢠⠀⠀⡠⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠁⣀⠔⠀⠀⠀⠀⠀⠀⠀⠛⠻⠟⠋⠀⠙⢦⠀⣠⠜⠀⠀
⠀⠈⠑⠤⡙⠳⣶⣦⣤⣤⣤⣤⣤⣤⣤⣤⣴⣶⡶⠞⠁⠀⠀⣠⠖⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠈⢯⠁⠀⠀⠀
⠀⠀⠀⠀⠈⢳⠤⣙⡻⠿⣿⣿⣿⣿⡿⠿⠛⠉⠀⢀⣀⡤⡚⠁⠀⠀⠀⠀⠀⠀⣧⠖⣁⣤⣦⠀⠀⠈⢇⠀⠀⠀
⠀⠀⠀⠀⠀⢸⠀⢀⣩⣍⠓⠒⣒⠒⠒⠒⠒⠊⠉⠁⢀⡟⠀⠀⣾⣷⠀⠀⠀⠀⠏⢴⣿⣿⣿⠀⠀⠀⢸⠀⠀⠀
⠀⠀⠀⠀⠀⠘⣶⣿⣿⣿⠀⠀⠈⠒⢄⣀⡀⠀⠀⠀⣼⣶⣿⡇⠈⠋⠀⠀⠀⡼⠀⠈⠻⣿⡿⠀⠀⠀⢸⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠹⡿⠿⠋⠀⠀⠀⠀⡜⠁⠈⢯⡀⢺⣿⣿⣿⠃⠀⠀⠀⢀⣼⣇⠀⠀⠀⠀⠀⠀⠀⠀⡞⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣿⣦⣄⣠⣀⣠⠞⠀⠀⠀⠈⠛⣿⡛⠛⠁⠀⠀⠀⣠⠊⠀⠈⢦⣄⣀⣀⣀⣀⢀⡼⠁⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠛⠉⠀⠀⠀⠀⠀⠀⠘⠛⠿⣿⠷⡾⠗⠊⠁⠀⠀⠀⠈⠉⠙⠛⠛⠛⠉⠀⠀

#0001 Bulbasaur ( 🌱 Grass / 🍄 Poison )
Height : 0.9 m, Weight : 6.9 kg, ♂ 87.5% --- ♀ 12.5%, Egg Group : Seed
Weaknesses : 🔥 Fire x2, ❄️  Ice x2 , 💨 Flying x2 and 🌀 Psychic x2
Resistances : 🌱 Grass x1/4, 💧 Water x1/2, ⚡️Electric x1/2, 👊 Fighting x1/2 and 🌸 Fairy x1/2
For some time after its birth, it uses the nutrients that are packed into the seed on its back in order to grow. 

<Bulbasaur> -> Lv. 16 -> Ivysaur -> Lv. 32 -> Venusaur
 
"),
        "Ivysaur" => println!("                                                         
                                                            
                      :-=+=:.                           
                   -******++++=:         -::=-.::       
                 .+******++++++*+:   .==+******+-.      
                :**+++**+++++++++*+-=+********+=-       
                +*+++++++++++++++************+-:        
               .**++++++++++++++***********+-:.         
   .---++=+*=-=-***+++++++++++********++++**+:          
 -++*************+*******+++********++++++-.            
.-++******************+===--=++=---:::-=++=--:          
 ..:=++++*************=+###+::::::.......:-::::.        
      :++=++++++*****++-#+-:::::+=:....:-....::::       
         .=++=+++++++**---:::: -**@*-.  ......::==      
             -=+=++****--::::: :+=+==:.........::=.     
               :=*****+-:::::::............:..::---:    
          .:=+****+=-=---:::::::.:::::::::--------:     
       .-+*****+=---+-:---------::----------------      
     -+***=+=.-----+=::::------------------====-.       
    :--=.:   .::-=---::::::--================--:        
            :+::------:::::::-=============-----        
            ==-:------:===-::--========++=--::--:       
            -:::--===---=+=-::-=+++++++-==-:::=-:       
            -:::--==:------:::--+++=-:  :=--:::-:       
                .-:--=-.   -=--:::--:.       -==-:-.        
                            --:-:-::                        
                                                            
#0002 Ivysaur ( 🌱 Grass / 🍄 Poison )
Height : 1 m, Weight : 13 kg, ♂ 87.5% --- ♀ 12.5%, Egg Group : Seed
Weaknesses : 🔥 Fire x2, ❄️  Ice x2 , 💨 Flying x2 and 🌀 Psychic x2
Resistances : 🌱 Grass x1/4, 💧 Water x1/2, ⚡️Electric x1/2, 👊 Fighting x1/2 and 🌸 Fairy x1/2
When the bulb on its back grows large, it appears to lose the ability to stand on its hind legs. 
                       
Bulbasaur -> Lv. 16 -> <Ivysaur> -> Lv. 32 -> Venusaur

"),
        _ => println!("
I don't know that pokemon.
Make sure you didn't make any typos or mistakes.
If you're sure you wrote it correctly, please make an issue on github.
"),  // Default case
    }
}