// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(item) = my_option {
        // my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);
    let len = 0;
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(len, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    let mut value_t = 0;
    // Let's swap these two!
    value_t = value_a;
    value_a = value_b;
    value_b = value_t;
    println!("value a: {}; value b: {}", value_a, value_b);
}
