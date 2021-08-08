const MAX_POINTS: u32 = 1000;


fn main() {
    let i: i64 = 1;
    println!("{}", i);
    let f = 1.1;    // default f64
    let a = 1;      // default i32
    let cc = '\u{1F601}';
    println!("{}", f);
    println!("{}", a);
    println!("{}", cc);
    println!("{}", MAX_POINTS);

    // while
    let max = 10;
    let mut num = 0;

    while num * num < max {
        println!("{0} * {0} = {1}", num, num * num);
        num += 1;
    }

    let mut x = 1;
    const MAX_NUM: i8 = 10;
    while x < MAX_NUM {
        // 允许重复定义相同变量名
        let mut y = x;
        while y < MAX_NUM {
            print!("{}*{}={} ", x, y, x * y);
            y += 1;
        }
        println!();
        x += 1;
    }

    let mut num = 0;
    // loop无限循环
    loop {
        println!("{0} * {0} = {1}", num, num * num);
        num += 1;
        if num * num > max {
            break;
        }
    }

    let t: bool = true;
    println!("{}", t);
    println!("--------------------------");

    // tuple
    let tup: (i32, i64) = (500, 1000);
    println!("{}, {}", tup.0, tup.1);
    let (x, y) = tup;
    println!("{}, {}", x, y);
    println!("--------------------------");

    // array
    let arr: [i32; 3] = [1, 2, 3];
    println!("{}", arr[0]);
    for elem in arr.iter() {
        println!("{}", elem)
    }
    println!("--------------------------");

    // 控制块
    let y = {
        let x = 1;
        x + 1
    };

    println!("{}", y);
    let num = get_num();
    println!("{}", num);

    let condition = true;
    let num = if condition {1} else {0};
    println!("{}", num);
    println!("--------------------------");

    range_num();

    let mut str = "hello";
    println!("{}", str);
    str = "rust";
    println!("{}", str);

}

fn get_num() -> i32 {
    6
}

fn range_num() {
    for num in 1..6 {       // 1-5
        println!("{}", num)
    }
}
