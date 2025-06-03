use std::io;
use std::collections::HashMap;


fn main() {
    let mut username = String::new();
    let mut enchantments: HashMap<String, String> = HashMap::new();

    println!("enter username");
    io::stdin().read_line(&mut username).unwrap();

    loop{
        let mut enchantment = String::new();
        let mut strength = String::new();
        println!("enter name of enchantment");
        println!("enter 'q' to quit");
        io::stdin().read_line(&mut enchantment).unwrap();
        if enchantment.trim() == "q"{
            break;
        }
        println!("enter strength of enchantment");
        io::stdin().read_line(&mut strength).unwrap();

        enchantments.insert(enchantment, strength);
    }

    let mut enchants_command = String::new();
    for (e,s) in &enchantments{
        enchants_command.push_str(&format!("'{}':{},",e.trim(),s.trim()));
    }
    enchants_command.pop();

    let command = format!("/give {} netherite_sword[enchantments={{{}}}] 1",username.trim(),enchants_command);

    println!("{}",command);
}