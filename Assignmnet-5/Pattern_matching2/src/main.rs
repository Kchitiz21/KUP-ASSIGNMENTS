#[derive(PartialEq, Eq, Debug)]
/// Passing coordinate.
enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}
#[derive(PartialEq, Eq, Debug)]
/// enum Position.
enum Positions {
    Origin(Coordinate, Coordinate),
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
}
/// This method shows the Quadrant Position.
///
/// #Arguments
///
/// Checking the Position.
///
/// #Return
///
/// Returns positions lies on which quadrant.
fn quadrant_positions(point: (i32, i32)) {
    match point {
        (x_axis, y_axis) if x_axis == 0 && y_axis == 0 => println!(
            "Position::Origin(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis > 0 && y_axis > 0 => println!(
            "Position::First(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis < 0 && y_axis > 0 => println!(
            "Position::Second(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis < 0 && y_axis < 0 => println!(
            "Position::Third(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis > 0 && y_axis < 0 => println!(
            "Position::Fourth(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        _ => println!("Incorrect"),
    }
}
/// This main method print the Coordinate numbers.
///
/// #Arguments
///
/// Find the Quadrant.
///
/// #Return
///
/// Returns successfully print quadrant position.
fn main() {
    let check_position = (2, -2);
    let check_position2 = (-1, -2);
    quadrant_positions(check_position);
    quadrant_positions(check_position2);
}