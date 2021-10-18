// Main function

fn main() {
    let numbers = Numbers {
        first_number: 30,
        second_number: 50,

    };
    // Numbers Structure
    
    struct Numbers {
        first_number: u32,
        second_number: u32,
    }
    
    // Implmenting Numbers Structure
    
    impl Numbers {
        
        
     //Function "add" adds numbers using structures to function
     // #Arguments
    //
    // Instance of the Structure Number
    //
    // #Return
    //
    // Returns addition of the two numbers
        
        fn add(&self) -> u32 {
            self.first_number + self.second_number

        }
        
    // Function "subtract" substract numbers using structures to function
    //
    // #Arguments
    //
    // Instance of the Structure Number
    //
    // #Return
    //
    // Returns subtraction of the two numbers
        
        fn subtract(&self) -> u32 {
            if self.first_number > self.second_number {
                self.first_number - self.second_number
            } else {
                self.second_number - self.first_number
            }
        }
        
    // Function "multiply" multiplies numbers using structures to function
    //
    // #Arguments
    //
    // Instance of the Structure Number
    //
    // #Return
    //
    // Returns product of the two numbers
        fn multiply(&self) -> u32 {
            self.first_number * self.second_number
        }
    }
}
