#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn area(rect: Rectangle) -> f32 {
    let Rectangle {
		top_left: Point { x: left_edge, y: top_edge },
		bottom_right: Point { x: right_edge, y: bottom_edge}
	} = rect;

	(right_edge - left_edge) * (top_edge - bottom_edge)
}

fn square(point: Point, length: f32) -> Rectangle {
    let Point { x: left_edge, y: top_edge } = point;
	Rectangle {
		top_left: Point { x: left_edge, y: top_edge },
		bottom_right: Point { x: left_edge + length, y: top_edge - length }
	}
}

fn main() {
    // Create struct with field init shorthand
    //let name = String::from("Peter");
    //let age = 27;
    //let peter = Person { name, age };

    // Print debug struct
    //println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, y: 2.4 };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: point,
        bottom_right: bottom_right,
    };

    let myArea = area(_rectangle);

	println!("area: {}", myArea);
    
    let next_point = Point { x: 3.3, y: 0.5 };
	let my_rect = square(next_point, 2.5);

    println!("square: ({}, {}) ({}, {})", my_rect.top_left.x, my_rect.top_left.y, my_rect.bottom_right.x, my_rect.bottom_right.y);
    // Instantiate a unit struct
    //let _unit = Unit;

    // Instantiate a tuple struct
    //let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    //println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    //let Pair(integer, decimal) = pair;

    //println!("pair contains {:?} and {:?}", integer, decimal);
}