#[derive(PartialEq, Eq, Debug)]
/// Passing Ip Class Name.
enum Ip {
    ClassA(usize),
    ClassB(usize),
    ClassC(usize),
    ClassD(usize),
    ClassE(usize),
}
/// List method shows the Ip format.
///
/// #Arguments
///
/// Checking the Position.
///
/// #Return
///
/// Returns Correct Ip Position.
fn list(position: (usize, usize, usize, usize)) {
    match position {
        (1..=126, 0..=255, 0..=255, 1..=254) => println!(
            "Ip::ClassA({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (128..=191, 0..=255, 0..=255, 1..=254) => println!(
            "Ip::ClassB({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (192..=223, 0..=255, 1..=254, 1..=254) => println!(
            "Ip::ClassC({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (224..=239, 0..=255, 0..=255, 0..=255) => println!(
            "Ip::ClassD({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (240..=254, 0..=255, 0..=255, 0..=254) => println!(
            "Ip::ClassE({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        _ => println!("Incorrect Number"),
    }
}
/// Main method print the Ip-Address.
///
/// #Arguments
///
/// Find the Ip number.
///
/// #Return
///
/// Returns successfully print Ip number.
fn main() {
    let address_1 = (192, 0, 1, 1);
    list(address_1);
    let address_2 = (230, 45, 6, 7);
    list(address_2);
    let address_3 = (170, 45, 23, 45);
    list(address_3);
    let address_4 = (198, 5, 6, 4);
    list(address_4);
}