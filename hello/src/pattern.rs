pub fn pattern() {
    /*
     * irrefutable
     * 1.for in
     * 2.let
     */

    /*
     * refutable
     * 1.match
     * 2.if let
     * 3.while let
     */

    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3..=5 => println!("three, four, or five"),
            _ => println!("anything"),
        }
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }

    {
        let s = Some(String::from("Hello!"));

        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }

    {
        let numbers = (1, 2, 3, 4);
        let (first, .., last) = numbers;
    }

    {
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => {
                println!("Found an id in range: {}", id_variable)
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }
        }
    }
}
