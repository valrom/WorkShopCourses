pub struct Watt(f32);

pub struct Celsius(f32);

mod smart {

    use super::{Celsius, Watt};
    pub struct _Socket {
        pub desc: String,
        pub power_consumption: Watt,
        pub state: bool,
    }

    impl _Socket {
        pub fn _get_desc(&self) -> &str {
            todo!()
        }

        pub fn _get_power_cons(&self) -> Watt {
            todo!()
        }

        pub fn _switch_on(&mut self) {
            todo!()
        }

        pub fn _switch_off(&mut self) {
            todo!()
        }
    }

    pub struct _Thermometer {
        pub temp: Celsius,
    }

    impl _Thermometer {
        pub fn _get_temp() -> Celsius {
            todo!()
        }
    }
}
