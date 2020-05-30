use std::num::NonZeroUsize;

fn main() {
    println!("Hello, world!");

    example1();
    example2();

    println!("{:?}", {
        let v1 = vec![1, 2, 3];
        let mut sum = 0;
        v1.into_iter().for_each(|x| sum += x);
        sum
    });

    {
        let x: NonZeroUsize = NonZeroUsize::new(1).unwrap();
        println!("{}", x);
    }
}

fn example1() {
    println!("example 1");

    let mut a1: [i32; 3] = [1, 2, 3];
    println!("a1 = {:?}", a1);

    {
        a1[2] = 5;
        println!("a1 = {:?}", a1);
        eprintln!("a1 = {:?}", a1);
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
        let iter = a1.iter(); // a1.into_iter();
        for x in iter {
            sum += x;
        }

        println!("a1 = {:?}, sum = {}", a1, sum);
    }
}

fn example2() {
    println!("example 2");

    // обычный for делает move
    {
        let v1: Vec<i32> = vec![1, 2, 3];
        println!("v1 = {:?}", v1);

        let mut sum = 0;
        for x in v1 {
            sum += x;
        }

        println!("sum = {}", sum);
    }

    // iter делает заимствование
    {
        let v1: Vec<i32> = vec![1, 2, 3];
        println!("v1 = {:?}", v1);

        let mut sum = 0;
        for x in v1.iter() {
            sum += x;
        }

        println!("v1 = {:?} sum = {}", v1, sum);
    }

    // iter_mut делает mut заимствование
    {
        let mut v1 = vec![1, 2, 3];
        println!("v1 = {:?}", v1);

        for x in v1.iter_mut() {
            *x += 1;
        }

        println!("v1 = {:?}", v1);
    }

    // push append
    {
        let mut v1 = vec![1, 2, 3];
        println!("v1 = {:?}, cap = {}", v1, v1.capacity());

        v1.push(4);
        println!("v1 = {:?}, cap = {}", v1, v1.capacity());

        let mut v2 = vec![10, 11, 12];
        v1.append(&mut v2);
        println!("v1 = {:?}, v2 = {:?}", v1, v2);
        println!("v1 cap = {}, v2 cap = {}", v1.capacity(), v2.capacity());
    }

    // capacity, index, pop
    {
        let mut v1 = Vec::with_capacity(10);
        println!("v1 = {:?}, cap = {}", v1, v1.capacity());

        v1.push(1);
        v1.push(2);
        v1.push(3);
        println!("v1 = {:?}, cap = {}", v1, v1.capacity());

        println!("v1[0] = {}", v1[0]);

        let last = v1.pop();
        println!("last = {:?}", last);

        println!("v1 = {:?}, cap = {}", v1, v1.capacity());

        let v2: Vec<_> = v1.iter().collect();
        println!("v2 = {:?}, cap = {}", v2, v2.capacity());
    }

    // iter
    {
        let mut v1 = vec![1, 2, 3];
        println!("v1 = {:?}, cap = {}", v1, v1.capacity());

        let v2: Vec<_> = v1.iter_mut().filter(|x| **x > 1).map(|x| *x * *x).collect();
        println!("v2 = {:?}, cap = {}", v2, v2.capacity());
    }
}
