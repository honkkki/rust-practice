#[derive(Debug)]
enum Cell {
    Int(i32),
    String(String),
}

fn get_largest_num_from_vec(list: &Vec<i32>) -> i32 {
    let mut res: i32 = list[0];
    for &num in list.iter() {
        if num > res {
            res = num;
        }
    }

    res
}

// 冒泡排序
fn bubble(list: &mut Vec<i32>) {
    let mut i = 0;
    while i < list.len() - 1 {
        let mut j = i + 1;
        while j < list.len() {
            if list[i] > list[j] {
                let tmp = list[i];
                list[i] = list[j];
                list[j] = tmp;
            }
            j += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut num = v[1];
    num = num + 1;
    println!("{} {:?}", num, v);

    for i in &mut v {
        *i += 10;
    }

    println!("{:?}", v);


    match v.get(1) {
        Some(num) => println!("index:1 num is {}", num),
        None => println!("no index:1 number"),
    }

    let mut v = Vec::new();
    v.push("hello");
    v.push("rust");
    v[1] = "golang";
    println!("{:?}", v);

    // vec and enum
    let v = vec![
        Cell::Int(1),
        Cell::String("orange".to_string()),
    ];

    println!("{:?}", v);
    println!("--------------------");

    let s = String::from("rust");
    let mut v = vec!["hello"];
    v.push(&s);
    println!("{:?}", v);
    println!("{}", s);
    println!("--------------------");

    let mut list = vec![3, 4, 5, 2, 1];
    let res = get_largest_num_from_vec(&list);
    println!("{}", res);
    println!("--------------------");

    bubble(&mut list);
    println!("{:?}", list)
}
