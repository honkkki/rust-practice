fn main() {
    let i: i64 = 1;
    println!("{}", i);
    let f = 1.1;    // default f64
    let a = 1;      // default i32
    let cc = '\u{1F601}';
    println!("{}", f);
    println!("{}", a);
    println!("{}", cc);

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
}
