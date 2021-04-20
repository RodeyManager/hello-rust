pub mod car {

    #[derive(Debug)]
    pub enum CarType {
        SmallCar,
        SUV,
        MVP,
    }

    #[derive(Debug)]
    pub struct Car {
        type_name: CarType,
        pub length: f32,
        pub width: f32,
        pub height: f32,
    }

    impl Car {
        pub fn new(type_name: CarType) -> Car {
            Car {
                type_name,
                length: 0.0,
                width: 0.0,
                height: 0.0,
            }
        }
    }
}
