extern crate rand;
use rand::Rng;

fn main() {
    let mut test_input: Vec<i32> = Vec::new();

    print!("Generating random list: ");

    for i in 0..10{
        test_input.push(rand::thread_rng().gen_range(0, 10));
        print!("{}{}", i, if i < 9 {','}else{'\n'});
    }

    println!("Sorting...");

    let mut sorted_vec: Vec<i32> = test_input.clone();
    let mut attempt = 1;
    while !check_sorted(&sorted_vec){
        attempt+=1;
        println!("not sorted, attempt {}, re-aranging...", attempt);
        sorted_vec.clear();
        populate_sorted_vec(&test_input, &mut sorted_vec);
    }

    for i in sorted_vec{
        println!("{}", i);
    }

}

fn populate_sorted_vec(source_vec: &Vec<i32>, dest_vec: &mut Vec<i32>){
    let mut temp_source = source_vec.clone();
    while temp_source.len()>0 {
        let current_len = temp_source.len();
        dest_vec.push(temp_source.remove(rand::thread_rng().gen_range(0, current_len)));
    }
}

fn check_sorted(check_vec: &Vec<i32>) -> bool{
    for i in 0..check_vec.len()-1{
        if check_vec.get(i) > check_vec.get(i+1){
            return false;
        }
    }
    true
}
