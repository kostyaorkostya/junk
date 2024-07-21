/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

type Floor = i32;

#[derive(Debug)]
enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    CarArrived(Floor),
    DoorOpened,
    DoorClosed,
    ButtonPressed(Button),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: Floor) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::DoorOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::DoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: Floor, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: Floor) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
