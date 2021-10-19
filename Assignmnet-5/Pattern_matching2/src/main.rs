#[derive(PartialEq, Eq, Debug)]
///enum Coordinate
///
enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}

///enum Position which used to describe the Position of Quadrant
///
enum Position {
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
    XAxis(Coordinate, Coordinate),
    YAxis(Coordinate, Coordinate),
    Origin(Coordinate, Coordinate),
}

/// Function 'check_coordinate' is used check the Quadrant of the given point
///
/// #Arguments
///
/// point: A point is tuple object of integer type
///
/// #Return
///
/// Returns the Position lied point
fn check_coordinate((point_1, point_2): (i32, i32)) {
    match (point_1, point_2) {
        (point_1, point_2) if point_1 > 0 && point_2 > 0 => println!(
            "Position::First(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 < 0 && point_2 > 0 => println!(
            "Position::First(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 < 0 && point_2 < 0 => println!(
            "Position::Third(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 > 0 && point_2 < 0 => println!(
            "Position::Fourth(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 == 0 && point_2 != 0 => println!(
            "Position::YAxis(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 != 0 && point_2 == 0 => println!(
            "Position::XAxis(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 == 0 && point_2 == 0 => println!(
            "Position::Origin(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        _ => println!("Wrong input"),
    }
}

/// Function main
fn main() {
    let first_point = (-2, -2);
    let second_point = (0, 0);
    check_coordinate(first_point);
    check_coordinate(second_point);
}
