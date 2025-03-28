#![allow(unused, dead_code)]

/*
 * Cargo Crates
 */
use std::collections::HashMap;
use std::env;

/*
 * Enums
 */
#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Fridge,
    Microwave,
    Toaster,
}

/*
 * Structs
 */
#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders: Vec<CustomerOrder> = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order: [u32; 6] = [2, 1, 2, 3, 4, 1];

    //let blender_orders: Vec<&CustomerOrder> = orders
    //    .iter()
    //    .filter(|order| {
    //        order.product == Product::Blender
    //    })
    //    .collect();
    //println!("{:?}", blender_orders);

    //let total_microwaves = orders
    //    .iter()
    //    .filter(|order| {
    //        order.product == Product::Microwave
    //    })
    //    .map(|microwave_order| {
    //        microwave_order.quantity
    //    })
    //    .fold(0, |total_microwaves, quantity| {
    //        total_microwaves + quantity
    //    });
    //println!("{:?}", total_microwaves);

    //let total_microwaves = orders
    //    .iter()
    //    .filter_map(|order| {
    //        if order.product == Product::Microwave {
    //            Some(order.quantity)
    //        } else {
    //            None
    //        }
    //    })
    //    .fold(0, |total_microwaves, quantity| {
    //        total_microwaves + quantity
    //    });
    //println!("{:?}", total_microwaves);

    //let total_microwaves = orders
    //    .iter()
    //    .filter(|order| {
    //        order.product == Product::Microwave
    //    })
    //    .fold(0, |total_microwaves, order| {
    //        total_microwaves + order.quantity
    //    });
    //println!("{:?}", total_microwaves);

    //let total_microwaves = orders
    //    .iter()
    //    .fold(0, |total_microwaves, order| {
    //        if order.product == Product::Microwave {
    //            total_microwaves + order.quantity
    //        } else {
    //            total_microwaves
    //        }
    //    });
    //println!("{:?}", total_microwaves);

    //let mut args = env::args().skip(1).take(1);

    //let mut args = args.map(|arg| {
    //    arg.parse::<u32>().unwrap_or(0)
    //});

    //let requested_quantity: u32 = args.next().unwrap_or(0);
    //println!("{:?}", requested_quantity);

    //let requested_orders: Vec<&CustomerOrder> = orders
    //    .iter()
    //    .filter(|order| order.quantity >= requested_quantity)
    //    .collect();
    //println!("{:?}", requested_orders);

    //let inventory = orders
    //    .iter()
    //    .filter(|order| !order.shipped)
    //    .fold(HashMap::<&Product, u32>::new(), |mut inventory, order| {
    //        let product_inventory: u32 = *inventory
    //            .get(&order.product)
    //            .unwrap_or(&0);

    //        let new_product_inventory: u32 = product_inventory
    //            + order.quantity;

    //        inventory.insert(&order.product, new_product_inventory);

    //        inventory
    //    });
    //println!("{:?}", inventory);

    //let mut first_unshipped: Option<CustomerOrder> = orders
    //    .into_iter()
    //    .find(|order| !order.shipped);
    ////println!("Order (before): {:?}", first_unshipped);

    //match first_unshipped {
    //    Some(ref mut order) => {
    //        order.shipped = true;
    //    }
    //    None => {
    //        println!("There is no unshipped order!");
    //    }
    //}
    //println!("Order (after): {:?}", first_unshipped);

    let mut customers: Vec<Customer> = customer_ids_by_order
        .iter()
        .zip(orders)
        .fold(
            HashMap::<u32, Vec<CustomerOrder>>::new(),
            |mut orders, combo| {
                let customer_id = combo.0;
                let order = combo.1;

                if let Some(customer_orders) = orders.get_mut(customer_id) {
                    customer_orders.push(order);
                } else {
                    orders.insert(*customer_id, Vec::<CustomerOrder>::from([order]));
                }

                orders
            })
        .into_iter()
        .map(|(customer_id, orders)| {
            Customer {
                id: customer_id,
                orders,
            }
        })
        .collect::<Vec<Customer>>();
    customers.sort_by_key(|customer| {
        customer.id
    });
    println!("{:#?}", customers);
}
