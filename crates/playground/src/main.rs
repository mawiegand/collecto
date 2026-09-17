struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    println!("Hello, world!");

    let list = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut index = 0;
    while index < list.len() {
        println!("list[{}] = {}", index, list[index]);
        index += 1;
    }

    for i in &list {
        println!("{}", i)
    }

    for i in list.iter().rev() {
        println!("{}", i)
    }

    let mut ind = list.iter();
    while ind.next() != None {
        println!("{:?}", ind);
    }

    let square = Rectangle {
        width: 2.25,
        height: 2.25,
    };
    println!("Square perimeter: {}", square.perimeter())
}
