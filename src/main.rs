fn main() {
    println!("Hello, world!");

    example1();
    example2();
}

fn example1() {
    println!("example 1");

    let mut a1: [i32; 3] = [1, 2, 3];
    println!("a1 = {:?}", a1);

    {
        a1[2] = 5;
        println!("a1 = {:?}", a1);
    }

    {
        let mut count = 0;
        'my_loop: loop {
            if count == a1.len() {
                break 'my_loop;
            }

            a1[count] += 1;

            count += 1;
        }

        println!("a1 = {:?}", a1);
    }

    {
        let mut count = 0;
        let sum = loop {
            count += 1;

            if count == 10 {
                break count * 2;
            }
        };

        println!("sum = {}", sum);
    }

    {
        let mut n = 0;
        while n < a1.len() {
            a1[n] += 10;

            n += 1;
        }

        println!("a1 = {:?}", a1);
    }

    {
        for i in 0..a1.len() {
            a1[i] *= a1[i];
        }

        println!("a1 = {:?}", a1);
    }

    {
        for i in 0..=2 {
            a1[i] /= 10;
        }

        println!("a1 = {:?}", a1);
    }

    {
        for x in &a1 {
            print!("{} ", x);
        }
        println!();
    }
    
    {
        for x in a1.iter() {
            print!("{} ", x);
        }
        println!();
    }

    {
        for x in a1.iter_mut() {
            if *x == 14 {
                *x = 9;
            }
        }

        println!("a1 = {:?}", a1);
    }

    {
        let mut sum = 0;
        let mut iter = a1.into_iter();
        for x in iter {
            sum += x;
        }

        println!("a1 = {:?}, sum = {}", a1, sum);
    }
}

fn example2() {
    println!("example 2");

    let v1: Vec<i32> = vec![1, 2, 3];
    println!("v1 = {:?}", v1)
}
