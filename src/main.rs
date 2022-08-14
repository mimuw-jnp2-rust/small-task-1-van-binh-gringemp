use std::fmt::{Display, Formatter};
use std::io::stdin;
use std::collections::HashMap;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
enum Dish {
    ThaiChicken,
    Tofu,
    FriedRice,
}

impl Dish {
    fn price(&self) -> u32 {
        match self {
            Dish::ThaiChicken => 20,
            Dish::Tofu => 15,
            Dish::FriedRice => 12,
        }
    }
}

const TAKEAWAY_FEE: u32 = 1;

#[derive(Debug, Clone)]
struct Order {
    takeaway: bool,
    name: String,
    map:  HashMap<Dish, u32>,
    item_count: u32
}



impl Order {
    fn new() -> Order {
        let result = Order {takeaway : false, name :  "".to_string(), map : HashMap::new(),
            item_count : 0};

        return result;
    }

    fn add_dish(&mut self, dish: Dish) {
        self.item_count += 1;
        match self.map.get(&dish)
        {
            None => self.map.insert(dish, 1),
            Some(value) => self.map.insert(dish, value + 1),
        };
    }

    fn set_takeaway(&mut self) {
        self.takeaway = true;
    }

    fn dish_count(&self, dish: Dish) -> u32 {
        match self.map.get(&dish)
        {
            None => return 0,
            Some(value) => return *value,
        };
    }

    fn items_count(&self) -> u32 {
        return self.item_count;
    }

    fn is_takeaway(&self) -> bool {

        return self.takeaway
    }

    fn total(&self) -> u32 {
        let mut a = 0;
        for dish in Dish::iter(){
            match self.map.get(&dish)
            {
                None => a  = a,
                Some(value) => a = a + value * dish.price(),
            };
        }
        let mut sum = a;

        if self.is_takeaway() {
            sum + self.items_count() * TAKEAWAY_FEE
        } else {
            sum
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "chicken: {}, tofu: {}, rice: {}, takeaway: {}",
            self.dish_count(Dish::ThaiChicken),
            self.dish_count(Dish::Tofu),
            self.dish_count(Dish::FriedRice),
            self.is_takeaway()
        )
    }
}

struct Customer {
    name: String,
    favorite_order: Order,
}

struct VanBinh {
    orders_count: u32,
    customers: Vec<Customer>,
}

impl VanBinh {
    pub fn new() -> VanBinh {
        return VanBinh{orders_count : 1, customers : Vec::new()}
    }

    fn add_customer(&mut self, name: String, favorite_order: Order) {
        self.customers.push(Customer{name,  favorite_order});
    }

    fn get_saved_customer(&self, name: &str) -> Option<&Customer> {
        self.customers.iter().find(|c| c.name == name)
    }

    fn increase_orders_count(&mut self) {
        self.orders_count += 1;
    }

    fn get_orders_count(&self) -> u32 {
        return self.orders_count
    }
}

fn get_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn yes_no(question: &str) -> bool {
    println!("{} (y/n)", question);
    get_line() == "y"
}

fn get_order() -> Order {
    let mut order = Order::new();
    loop {
        println!("Enter dish name or empty line to finish:");
        let line = get_line();
        if line.is_empty() {
            break;
        }
        if line.contains("chicken") {
            order.add_dish(Dish::ThaiChicken);
        } else if line.contains("tofu") {
            order.add_dish(Dish::Tofu);
        } else if line.contains("rice") {
            order.add_dish(Dish::FriedRice);
        } else {
            println!("Unknown dish name: {}", line);
        }
    }

    if yes_no("Takeaway?") {
        order.set_takeaway();
    }

    order
}

fn main() {
    let mut van_binh = VanBinh::new();

    loop {
        println!("Hi! Welcome to Van Binh! What's your name?");
        let name = get_line();

        if name.is_empty() {
            break;
        }

        let order = if let Some(customer) = van_binh.get_saved_customer(&name) {
            println!("Welcome back, {}!", customer.name);
            if yes_no("Same as usual?") {
                println!("{}", customer.favorite_order);
                customer.favorite_order.clone()
            } else {
                get_order()
            }
        } else {
            println!("Welcome, {}!", name);
            let order = get_order();
            if yes_no("Would you like to save this order?") {
                van_binh.customers.push(Customer{name, favorite_order : order.clone()}) // save customer's favorite order in van_binh struct
            }
            order
        };
        if order.item_count == 0 { // Check if the order is empty
            println!("Your order is empty!");
        }

        println!("This is order no. {}", van_binh.get_orders_count());
        println!(
            "There you go: {}, it's going to be {} zł",
            order,
            order.total()
        );
        van_binh.increase_orders_count();
    }
    println!("Bye!");
}
