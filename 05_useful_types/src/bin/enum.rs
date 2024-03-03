#[derive(Debug)]
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum TravelAllowance {
    Bus(u32),
    Train(u32),
    Airplane(u32),
}

impl TravelAllowance {
    fn caculate(&self) -> u32 {
        let allowance = match self {
            TravelAllowance::Bus(miles) => miles * 5,
            TravelAllowance::Train(miles) => miles * 15,
            TravelAllowance::Airplane(miles) => miles * 40,
        };
        allowance
    }
}

fn main() {
    let today = WeekDay::Saturday;
    match today {
        WeekDay::Monday => println!("Today is Monday"),
        WeekDay::Tuesday => println!("Today is Tuesday"),
        WeekDay::Wednesday => println!("Today is Wednesday"),
        WeekDay::Thursday => println!("Today is Thursday"),
        WeekDay::Friday => println!("Today is Friday"),
        WeekDay::Saturday => println!("Today is Saturday"),
        WeekDay::Sunday => println!("Today is Sunday"),
    }

    let allowance = TravelAllowance::Bus(100);
    println!("Allowance: {}", allowance.caculate());
}
