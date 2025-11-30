use std::fs::File;
use std::io::Write;

fn main() {
    let drink_categories = "Lager :\n- Desperadoes\n- Gulder\n- Heineken\n- Star\n\nStout
    :\n- Legend\n- Turbo King\n- Williams\n\nNon-Alcoholic
    :\n- Maltina\n- Amstel Malt\n- Malta Gold\n- Fayrouz";

    // Creating a file 
    let mut file = std::fs::File::create("nigerian_breweries_drinks.txt").expect("Could not create file");

    file.write_all(drink_categories.as_bytes()).expect("could not write to file");

    println!("File successfully created");
}
