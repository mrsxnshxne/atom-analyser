mod atom_analyser;

fn main() {

    println!("Welcome to Atom-Analyser.");
    println!("--------------------------");
    println!("1. Destructure compound.");
    println!("--------------------------");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    let choice: u32 = choice.trim().parse().unwrap();

    match choice {
        1 => {
            println!("Enter the compound: ");
            let mut compound = String::new();
            std::io::stdin().read_line(&mut compound).unwrap();
            let compound = compound.trim();

            let compound = atom_analyser::destructure_compound(compound);

            println!("{:?}", compound);
        },

        _ => {
            println!("Invalid choice.");
        }
    }

}
