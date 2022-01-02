#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
    z: f64,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65, 30];
    let number_list_float = vec![1.1, 2.2, 3.3];
    let num = largest(&number_list);
    let largest_float = largest(&number_list_float);
    println!("largest num: {}", num);
    println!("largest float: {}", largest_float);

    let p = Point {
        x: 1,
        y: 2,
        z: 1.1,
    };

    println!("{:#?}", p);
    println!("x: {}", p.get_x())
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
