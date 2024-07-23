use gilrs::{Gilrs, Event, EventType, Button};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    for (id, gamepad) in gilrs.gamepads() {
        println!("{} id={id} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        while let Some(event) = gilrs.next_event() {
            match event {
                // Event { event: EventType::ButtonPressed(Button::South, _), .. } => {
                //     println!("pressed: X")
                // }
                Event { event: EventType::AxisChanged(axis, val, _), .. } if val.abs() > 0.2 => {
                    println!("axis {:?} : {:?}", axis, val)
                }
                Event { event: EventType::ButtonChanged(Button::RightTrigger2, val, _), .. } => {
                    println!("pressing: R2 :: {val}")
                }
                Event { event: EventType::ButtonChanged(Button::LeftTrigger2, val, _), .. } => {
                    println!("pressing: R2 :: {val}")
                }
                Event { event: EventType::ButtonPressed(btn, _), .. } => {
                    println!("pressed: {:?}", btn);
                }
                Event { id, event: EventType::Connected, .. } => {
                    println!("Gamepad connected :: id={id}")
                }
                Event { id, event: EventType::Disconnected, .. } => {
                    println!("Gamepad disconnected :: id={id}")
                }
                _ => (),
            };
        }
    }
}
