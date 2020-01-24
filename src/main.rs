#[derive(Clone, Copy)]
struct Pixel {
    red: u8,
    blue: u8,
    green: u8
}

// struct Image {
//     File: vec<Pixel>
// }

fn main() {
    println!("\nHELLO WORLD!")
}

impl Pixel {
    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            red: red,
            green: green,
            blue: blue 
        }
    }

    fn red(&self) -> u8 {
        self.red
    }

    fn green(&self) -> u8 {
        self.green
    }

    fn blue(&self) -> u8 {
        self.blue
    }

    fn display(self) -> String {
        println!("color")
    }

    // Invert (negative)
    // R = 255 - R
    // G = 255 - R
    // B = 255 - B

    // Greyscale
    // Grey = R / 3 + G / 3 + B / 3
}


