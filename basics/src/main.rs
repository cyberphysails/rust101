fn e_types() {
    // 最大值 和 位宽
    println!("i32 max = {}", i32::MAX);
    println!("u32 max = {}", u32::MAX);
    println!("i64 max = {}", i64::MAX);
    println!("u64 max = {}", u64::MAX);
    println!("isize max = {}", isize::MAX);
    println!("isize bytes width: {}", std::mem::size_of::<isize>());
    println!("usize max = {}", usize::MAX);
    println!("usize bytes width: {}", std::mem::size_of::<usize>());
}

fn e_loop() {
    let a = 39;
    let mut factor = 2;
    let res = loop {
        if (a % factor) == 0 {
            break false;
        };
        factor += 1;
        if ( a - 1 ) == factor {
            break true;
        };
    };

    println!("{a} is a prime number? {res}");
}

fn e_tuples() {
    let t = (1, "hi", false);
    println!("tuple is {:?}", t);
    println!("tuple eles: {},{},{}", t.0, t.1, t.2);

    let (first, sec, third) = t;
    println!("first : {}", first);
    println!("second : {}", sec);
    println!("third : {}", third);
}

fn e_arrays() {
    let a = [3; 5];
    let mut i = 0;
    for ele in a {
        println!("elements {i} is {ele}");
        i += 1;
    }
}

fn e_break_label() {
    let s = [[1,2,3],[4,5,6], [7,8,9]];
    let target = 6;
    let mut ele_searched = 0;
    'outer: for j in s {
       for k in j {
            if k == target {
                break 'outer;   
            }else {
                ele_searched += 1;
            }
        } 
    };
    println!("Searched {} items!", ele_searched); 
}

fn factorial(n: u32) -> u32 {
   let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn e_macros_test() {
    let n = 4;
    println!("factorial of {} is {}", n, factorial(n));
}

fn main() {
    //e_arrays();
    //e_break_label();
    e_macros_test();
}
