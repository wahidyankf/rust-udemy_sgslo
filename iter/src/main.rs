fn print_elements_for(elements: &Vec<String>) {
    for element in elements.iter() {
        println!("{}", element);
    }
}

fn print_elements_for_each(elements: &Vec<String>) {
    elements.iter().for_each(|element| {
        println!("{}", element);
    });
}

fn print_elements_iter_adaptor(elements: &Vec<String>) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut colors_iter = colors.iter();

    println!("colors: {:?}", colors_iter);
    println!("colors: {:?}", colors_iter.next());
    println!("colors: {:?}", colors_iter.next());
    println!("colors: {:?}", colors_iter.next());
    println!("colors: {:?}", colors_iter.next());

    println!("---");

    print_elements_for(&colors);
    println!("---");
    print_elements_for_each(&colors);
    println!("---");
    print_elements_iter_adaptor(&colors);
    println!("---");
}
