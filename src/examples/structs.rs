/* Flatten a struct into a 1-order list of fields */

#[derive(Debug)]
pub struct Restaurant {
    pub name: String,
    pub employees: u16,
    pub restaurantType: ResType,
    pub front_of_house: FrontHouse,
    pub back_of_house: BackHouse,
}
    #[derive(Debug)]
    pub enum ResType {
        FastFood,
        FamilyDinner,
        FineDining,
        DinnerWithShow,
        Deli,
    }
    #[derive(Debug)]
    pub struct FrontHouse {
        pub employees: u8,
        pub head_waiter: Option<String>,
        pub tables: Vec<Table> 
    }

        #[derive(Debug)]
        pub struct Table {
            pub table_capacity: u8,
            pub table_quality: TableQuality
        }

            #[derive(Debug)]
            pub enum TableQuality { 
                New(String),
                Good(String),
                Bad(String),
                OutofOrder(chrono::NaiveDateTime)
            }
    #[derive(Debug)]
    pub struct BackHouse {
        pub employees: u8,
        pub head_chef: String,
    }

impl Restaurant {
    pub fn example_restaurant() -> Restaurant {

        Restaurant{
            name: "Chuck's".to_string(),
            employees: 24,
            restaurantType: ResType::FineDining,
            front_of_house: FrontHouse {
                employees: 14,
                head_waiter: None,
                tables: vec![
                    Table  {
                        table_capacity: 2,
                        table_quality: TableQuality::Good("It only wobbles when it rains".to_string())
                        }, 
                    Table { 
                        table_capacity: 6,
                        table_quality: TableQuality::Bad("Only three legs".to_string())
                        },
                    ]
            },
            back_of_house: BackHouse {
                employees: 4,
                head_chef: "Rocko".to_string(),
            }
        }
    }
}

pub fn navigate_struct() -> () {

    let chucks = Restaurant::example_restaurant();

    println!("Chuck's Head Waiter is: {:?}", chucks.front_of_house.head_waiter);
    println!("\nChuck's Head Chef is: {:?}", chucks.back_of_house.head_chef);
    println!("\nChuck's Best Table is: {:?}", chucks.front_of_house.tables[0]);
    println!("\nChuck's restaurant type is: {:?}", chucks.restaurantType);
    println!("\n\nChuck's altogether is: {:?}", chucks);

}

