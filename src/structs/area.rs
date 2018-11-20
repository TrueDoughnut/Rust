pub fn run(){
    let rect = create_rect(50, 30);
    println!("{}", rect.area());

    let square = Rect::square(30);
    println!("{}", square.area());

}

struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: Rect) -> bool {
        self.height > rect.height && self.width > rect.width
    }

    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size
        }
    }
}

fn create_rect(width: u32, height: u32) -> Rect {
    Rect {
        width,
        height
    }
}
