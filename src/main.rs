type Predicate<T> = Box<dyn Fn(&T) -> bool>;

struct Person {
    name: String,
}

fn main() {
    let buffer = (0..100).collect::<Vec<i32>>();
    println!("{buffer:?}");

    let result = buffer
        .into_iter()
        .filter(|p| *p % 2 == 0)
        .collect::<Vec<i32>>();

    println!("{result:?}");
    let result = add_curried(1)(2)(3);
    println!("result is {result}");
    let sum_one = add_curried(1);
    let result = sum_one(4)(3);
    println!("result is {result}");

    let buffer = vec![1, 2, 3, 4, 5, 6];
    let result = filter(buffer, Box::new(|v| v % 2 == 0));

    println!("result is {result:?}");

    let mvalue = 5;
    let increment = |x| x + mvalue;
    let result = increment(1);
    println!("{result}");

    let person = Person {
        name: "Julio".to_owned(),
    };

    // try moving the captured person variable: eg. move |message: &str| ...
    let print = |message: &str| println!("{message}, {}", person.name);
    print("Hello");
    println!("{}", person.name);

    let result = calc(10, 2, |a, b| a + b);
    println!("{result}");

    let mut counter = count_from(0);
    {println!("{}", counter());}
    {println!("{}", counter());}
    {println!("{}", counter());}
    {println!("{}", counter());}
    
    let result = calc(10, 4, |a, b| a - b);
    println!("{result}");
    let result = (1..=100).into_iter()
    .map(|d|{
        d.to_string()
    })
    .collect::<Vec<String>>();
    
    println!("{result:?}");
}

fn calc<F>(a: i32, b: i32, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(a, b)
}

fn add_curried(a: i32) -> impl FnOnce(i32) -> Box<dyn FnOnce(i32) -> i32> {
    move |b| Box::new(move |c| a + b + c)
}

fn filter<A>(input: Vec<A>, f: Predicate<A>) -> Vec<A> {
    let mut output = Vec::<A>::new();
    for v in input {
        if f(&v) {
            output.push(v);
        }
    }
    output
}

fn count_from(start: i32) -> impl FnMut() -> i32 {
    let mut s = start;
    move ||{
        s += 1;
        s
    }
}
