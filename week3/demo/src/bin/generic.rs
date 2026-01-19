use std::fmt::{self, write, Display};
struct MatchingPair<T> {
    first: T,
    second: T
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {x: x, y: y}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y )
    }
}
/* <T: Copy> means MatchingPair<T> can only implement Clone 
    if T itself implements Clone. */
impl<T: Copy> Clone for MatchingPair<T> {
    fn clone(&self) -> Self {
        MatchingPair::new(self.first, self.second)
    }
}

pub enum MyOption<T> {
    Sumthin(T), Nuthin
}

impl fmt::Display for MyOption<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyOption::Sumthin(num) => write!(f, "Sumthin({})", num),
            MyOption::Nuthin => write!(f, "Nuthin :("),
        }
    }    
}

impl<T> MatchingPair<T> {
    pub fn new(first: T, second: T) ->Self {
        MatchingPair {first: first, second: second}
    }
}

impl<T> fmt::Display for MatchingPair<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second )
    }
}

/* generic function */
fn identity_fn<T>(x: T) -> T{x}

/* T must impl Display */
fn print_excited<T: fmt::Display>(x: T) {
    println!("{}!!!!!!!!!!! :DDDDDDD", x);
}

/* T must impl Display and PartialOrd */
fn print_min<T: fmt::Display + PartialOrd>(x: T, y: T) {
    if x < y {
        println!("The minimum is {}", x);
    } else {
        println!("The minimum is {}", y);
    }
}

fn main() {
    let ps_in_a_pod: MatchingPair<char> = MatchingPair::new('p', 'P');
    println!("two ps in a pod: {}", ps_in_a_pod);
    let zs_in_a_pod: MatchingPair<u32> = MatchingPair::new(2, 5);
    println!("zs in a pod: {}", zs_in_a_pod);  
    let my_some_five: MyOption<u32> = MyOption::Sumthin(5);
    let my_nuthin: MyOption<u32> = MyOption::Nuthin;
    println!("my_some_five: {}", my_some_five);
    println!("my_nuthin: {}", my_nuthin);

    let clone_ps_in_a_pod = ps_in_a_pod.clone();
    println!("clone_ps_in_a_pod: {}", clone_ps_in_a_pod);

    let clone_pair_of_points = MatchingPair::new(Point::new(1.0,1.0), Point::new(2.0,2.0)).clone();
    println!("clone_pair_of_points: {}", clone_pair_of_points);

    println!("{:?}", identity_fn(Point::new(1.0, 1.0)));
    print_excited(Point::new(2.0, 2.0));
    print_min(10, 5);

}