fn main() {
    let array: [i32; 100] = [
        58, 20, 77, 33, 10, 99, 50, 30, 2, 75,
        88, 60, 37, 89, 9, 70, 36, 14, 5, 34,
        38, 92, 40, 64, 31, 21, 79, 78, 13, 83,
        25, 71, 41, 15, 93, 18, 46, 44, 6, 28,
        51, 22, 80, 1, 7, 39, 49, 17, 90, 67,
        26, 84, 3, 45, 72, 11, 96, 29, 65, 32,
        94, 55, 76, 4, 85, 42, 66, 8, 23, 52,
        12, 97, 73, 35, 86, 43, 68, 16, 24, 53,
        19, 98, 74, 47, 87, 56, 69, 48, 27, 54,
        81, 100, 82, 57, 91, 62, 95, 63, 61, 59
    ];
    let greeter: i32 = return_greatest(&array);
    println!("{}", greeter);
}

fn exec_bubble_sort(){
    let mut array: [i32; 100] = [
        58, 20, 77, 33, 10, 99, 50, 30, 2, 75,
        88, 60, 37, 89, 9, 70, 36, 14, 5, 34,
        38, 92, 40, 64, 31, 21, 79, 78, 13, 83,
        25, 71, 41, 15, 93, 18, 46, 44, 6, 28,
        51, 22, 80, 1, 7, 39, 49, 17, 90, 67,
        26, 84, 3, 45, 72, 11, 96, 29, 65, 32,
        94, 55, 76, 4, 85, 42, 66, 8, 23, 52,
        12, 97, 73, 35, 86, 43, 68, 16, 24, 53,
        19, 98, 74, 47, 87, 56, 69, 48, 27, 54,
        81, 100, 82, 57, 91, 62, 95, 63, 61, 59
    ];
        println!("Before -> {:?}", array);
        bubble_sort(&mut array);
        println!("Late -> {:?}", array)
}

fn bubble_sort(array: &mut [i32]){
    let is_ascending: bool = array.windows(2).all(|w| w[0] <= w[1]);
    if is_ascending {
        return
    } else {
        for i in 1..array.len(){
            if array[i - 1] >= array[i]{
                array.swap(i - 1, i);
            }
        }
        bubble_sort(array);
    }
}

fn return_greatest(array: &[i32]) -> i32 {
    let mut greeter: i32 = array[0];
    let mut test: Vec<i32> = Vec::new();
    test.pu
    for i in 1..array.len(){
        if array[i - 1] >= greeter{
            greeter = array[i - 1];
        }
    }
    greeter
}

fn is_prime(num: i32) {
    if num % 2 == 0 || num == 2{
        return println!(" é um primo");
    }
    let mut numbers_list: Vec<i32> = vec![];
    for i in 1..=num{
        numbers_list.push(i);
    }
    let mut attemps: i32 = 0;
    for i in numbers_list{
        if num % i == 0 {
            attemps += 1;
        }
    }
    if attemps >= 4{
        println!(" não é um primo");
    }
    else if attemps == 2{
        println!(" é um primo");
    }
}
