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
    /*
     * Data coming from two different sources, but matched by shared index
     * position.
     */
    // Product Orders:
    let mut orders: Vec<CustomerOrder> = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    // Customers:
    let customer_ids_by_order: [u32; 6] = [2, 1, 2, 3, 4, 1];

    //let blender_orders: Vec<&CustomerOrder> = orders
    //    /*
    //     * Iterator that yields immutable references. We never take ownership of
    //     * the `orders` vector, neither of its individual `customer_orders`.
    //     */
    //    .iter()
    //    .filter(|order| { // Here, we actually filter references.
    //        order.product == Product::Blender
    //    })
    //.collect(); // Here, we collect those references.
    //println!("{:?}", blender_orders);

    //let microwave_count = orders
    //    .iter()
    //    .filter(|order| {
    //        order.product == Product::Microwave
    //    })
    //    .map(|microwave_order| {
    //        microwave_order.quantity
    //    })
    //    .fold(0, |microwave_count, quantity| {
    //        microwave_count + quantity
    //    });
    //println!("{:?}", microwave_count);

    //let microwave_count = orders
    //    .iter()
    //    .filter_map(|order| {
    //        if order.product == Product::Microwave {
    //            Some(order.quantity)
    //        } else {
    //            None
    //        }
    //    })
    //    .fold(0, |microwave_count, quantity| {
    //        microwave_count + quantity
    //    });
    //println!("{:?}", microwave_count);

    //let microwave_count = orders
    //    .iter()
    //    .filter(|order| {
    //        order.product == Product::Microwave
    //    })
    //    .fold(0, |microwave_count, order| {
    //        microwave_count + order.quantity
    //    });
    //println!("{:?}", microwave_count);

    //let microwave_count = orders
    //    .iter()
    //    .fold(0, |microwave_count, order| {
    //        if order.product == Product::Microwave {
    //            microwave_count + order.quantity
    //        } else {
    //            microwave_count
    //        }
    //    });
    //println!("{:?}", microwave_count);

    //let microwave_count: u32 = orders
    //    .iter()
    //    .filter(|order| {
    //        order.product == Product::Microwave
    //    })
    //    .map(|microwave_order| {
    //        microwave_order.quantity
    //    })
    //    .sum();
    //println!("{:?}", microwave_count);

    //let microwave_count = orders
    //    .iter()
    //    .filter_map(|order| {
    //        if order.product == Product::Microwave { // "filter" part
    //            Some(order.quantity) // "map" part
    //        } else {
    //            None
    //        }
    //    })
    //    .sum::<u32>();
    //println!("{:?}", microwave_count);

    //let mut args = env::args().skip(1).take(1);
    //
    //let mut args = args.map(|arg| {
    //    arg.parse::<u32>().unwrap_or(0)
    //});
    //
    //let user_quantity: u32 = args.next().unwrap_or(0);
    //println!("{:?}", user_quantity);
    //
    //let orders_by_quantity: Vec<&CustomerOrder> = orders
    //    .iter()
    //    .filter(|order| order.quantity >= user_quantity)
    //    .collect();
    //println!("{:?}", orders_by_quantity);

    //let mut user_quantity = env::args()
    //    .skip(1)
    //    .take(1)
    //    .map(|quantity| quantity.parse::<u32>().unwrap_or(0))
    //    .next()
    //    .unwrap_or(2);
    //
    //let orders_by_quantity: Vec<&CustomerOrder> = orders
    //    .iter()
    //    .filter(|order| order.quantity >= user_quantity)
    //    .collect();
    //println!("{:?}", orders_by_quantity);

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

    //let inventory = orders
    //    .iter()
    //    .filter(|order| !order.shipped)
    //    .fold(HashMap::<&Product, u32>::new(), |mut inventory, order| {
    //        let entry = inventory.entry(&order.product).or_insert(0);

    //        *entry += order.quantity;

    //        inventory
    //    });
    //println!("{:?}", inventory);

    //let mut first_unshipped: Option<CustomerOrder> = orders
    //    .into_iter()
    //    .find(|order| !order.shipped);
    //println!("Order (before): {:?}", first_unshipped);

    //match first_unshipped {
    //    Some(ref mut order) => {
    //        order.shipped = true;
    //    }
    //    None => {
    //        println!("There is no unshipped order!");
    //    }
    //}
    //println!("Order (after): {:?}", first_unshipped);

    //if let Some(first_unshipped) = orders
    ////if let Some(first_unshipped: &mut CustomerOrder) = orders
    //    .iter_mut()
    //    .find(|order| !order.shipped) {
    //        first_unshipped.shipped = true;
    //    }
    //println!("Orders: {:?}", orders);

    //let mut customers: Vec<Customer> = customer_ids_by_order
    //    .iter()
    //    .zip(orders)
    //    .fold(
    //        HashMap::<u32, Vec<CustomerOrder>>::new(),
    //        |mut orders, combo| {
    //            let customer_id = combo.0;
    //            let order = combo.1;

    //            if let Some(customer_orders) = orders.get_mut(customer_id) {
    //                customer_orders.push(order);
    //            } else {
    //                orders.insert(*customer_id, Vec::<CustomerOrder>::from([order]));
    //            }

    //            orders
    //        })
    //    .into_iter()
    //    .map(|(customer_id, orders)| {
    //        Customer {
    //            id: customer_id,
    //            orders,
    //        }
    //    })
    //    .collect::<Vec<Customer>>();
    //customers.sort_by_key(|customer| {
    //    customer.id
    //});
    //println!("{:#?}", customers);

    let mut customers: Vec<Customer> = orders
        // Iterator that takes full ownership of the `CustomerOrder` structs.
        .into_iter()
        .zip(customer_ids_by_order)
        .fold(
            HashMap::<u32, Vec<CustomerOrder>>::new(),
            |mut ids_to_orders, (order, customer_id)| {
                let mut orders: &mut Vec<CustomerOrder> = ids_to_orders
                    .entry(customer_id)
                    .or_insert(vec![]);

                orders.push(order);

                ids_to_orders
            }) // HashMap<u32, Vec<CustomerOrder>>
        .into_iter()
        .map(|(customer_id, orders)| {
            Customer {
                id: customer_id,
                orders,
            }
        })
        .collect::<Vec<Customer>>();

    customers.sort_by_key(|customer| customer.id);

    println!("{:#?}", customers);
}
