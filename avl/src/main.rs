mod avl;

use avl::AVL;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut tree = AVL::new();
    loop {
        let mut s = String::new();
        print!("insert/delete/print/height/num_leaves/is_empty/inorder: ");
        stdout().flush();
        stdin().read_line(&mut s).expect("Incorrect Command");

        s = String::from(s.trim());

        let mut input_iter = s.split(" ");
        let input = input_iter.next().unwrap();

        match input {
            "insert" => {
                let val = input_iter.next();

                if val.is_none() {
                    println!("\nInvalid Command, try again\n");
                    continue;
                }
                let val_int: i32 = val.unwrap().parse().unwrap();
                tree.insert(val_int);
            }
            "height" => {
                println!("\nTree Height: {}\n", tree.height());
            }
            "num_leaves" => {
                println!("\nNumber of leaves: {}\n", tree.count_leaves());
            }
            "is_empty" => {
                println!(
                    "\nTree is {}empty\n",
                    if tree.isEmpty() { "" } else { "not " }
                );
            }
            "inorder" => {
                tree.inorder();
            }
            "delete" => {
                let val = input_iter.next();

                if val.is_none() {
                    println!("Invalid Command, try again");
                    continue;
                }

                let val_int: i32 = val.unwrap().parse().unwrap();

                tree.delete(val_int);
            }
            "print" => {
                println!("Tree Pretty Printed: \n{}", tree.print());
            }
            "close" => {
                return;
            }
            _ => {
                println!("\nInvalid Command\n");
            }
        };
    }
}
