
fn main() {
    let numbers = Numbers {
        first_number: 30,
        second_number: 50,

    };
    struct Numbers {
        first_number: u32,
        second_number: u32,
    }

    impl Numbers {
        fn add(&self) -> u32 {
            self.first_number + self.second_number

        }
        fn subtract(&self) -> u32 {
            if self.first_number > self.second_number {
                self.first_number - self.second_number
            } else {
                self.second_number - self.first_number
            }
        }
        fn multiply(&self) -> u32 {
            self.first_number * self.second_number
        }
    }
}