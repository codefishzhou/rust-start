fn main() {
    let mut count = 0;
    loop {
        println!("again!, {}", count);
        count += 1;
        if count == 10 {
            break;
        }
    }
    main1();
}

fn main1() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}