// Problem 4: Fix the struct definition to allow closures with event handling logic.

struct EventHandler<T>
where
    T: FnMut() -> (), // Something wrong here.
                      //Hint: Check the code in main and see how the closure is using
                      // the values from its enviroment
{
    on_event: T,
}

impl<T> EventHandler<T>
where
    T: FnMut() -> (), // Something wrong here
{
    fn handle_event(&mut self) {
        (self.on_event)()
    }
}

fn main() {
    let mut lights_on = false;
    let mut temperature = 25;

    let mut lights_handler = EventHandler {
        on_event: || {
            lights_on = !lights_on;
            println!("Lights are now {}", if lights_on { "on" } else { "off" });
        },
    };

    let mut temperature_handler = EventHandler {
        on_event: || {
            temperature += 5;
            println!("Temperature increased to {}°C", temperature);
        },
    };

    lights_handler.handle_event();
    temperature_handler.handle_event();
    temperature_handler.handle_event();
    lights_handler.handle_event();

    assert_eq!(temperature, 35);
    assert_eq!(lights_on, true);
}
