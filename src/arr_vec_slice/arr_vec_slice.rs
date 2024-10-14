fn main () {
    let a: [i32; 5] = [0, 1, 2, 3, 4];

    for i in a {
        println!("{}", i);
    }

    //println!("{}", a[5]); 
    
    let mut v: Vec<i32> = vec![0, 1, 2, 3, 4];

    for i in &v {
        println!("{}", i);
    }

    v.push(5);

    for i in &v {
        println!("{}", i);
    }

    let s: &[i32] = &a[1..4];

    for i in s {
        println!("{}", i);
    }

    let sv: &[i32] = &v[3..];

    for i in sv {
        println!("{}", i);
    }

    sv.push(6);

    //for i in sv {
    //    println!("{}", i);
    //}
}
