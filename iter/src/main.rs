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

fn print_elements_vec_slice(elements: &[String]) {
    for element in elements {
        println!("{}", element);
    }
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut colors_mut = colors.clone();

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
    print_elements_vec_slice(&colors[1..3]);
    println!("---");

    shorten_strings(&mut colors_mut);
    println!("{:?}", colors);
    println!("---");

    shorten_strings(&mut colors_mut[1..3]);
    println!("{:?}", colors);
    println!("---");

    let uppercased = to_uppercase(&colors);
    println!("{:?}", uppercased);
    println!("---");

    let colors_to_move = colors.clone();

    let mut destination = vec![];
    move_elements(colors_to_move, &mut destination);

    println!("Destination: {:#?}", destination);
    println!("---");
}
