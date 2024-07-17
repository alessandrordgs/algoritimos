#[derive(Debug)]
enum IntOrString<'a> {
    Int(i32),
    Str(&'a str)
}
fn binary_search(mut list: Vec<i32>, item: i32) -> IntOrString<'static>  {
    let mut low_index: i32 = 0;
    list.sort();
    let mut high_index: i32 = (list.len() - 1).try_into().unwrap();
    while low_index <= high_index {
        let middle = (low_index + high_index) / 2;
        let kick = list[middle as usize];
        if kick == item {
            return IntOrString::Int(middle);
        }

        if kick > item {
            high_index = middle - 1
        } else {
            low_index = middle + 1
        }
    }
    return IntOrString::Str("Not Found");
    // println!("{:#?}", list)
}

fn main() {
    let list_of_numbers = vec![2, 10, 1, 5, 10, 6, 7];
    let result = binary_search(list_of_numbers, 6);

    println!("{:#?}", result)
}
