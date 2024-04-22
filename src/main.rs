fn bubble_sort<T>(a: &mut[T])
    where
        T:PartialOrd+Ord+Sized
{
    let size = a.len();
    let mut swapped = true;
    if size > 1 {
        for i in 0..=size - 2 {
            if !swapped {
                break
            }
            swapped = false;
            for j in 0..=size - i - 2 {
                if a[j] > a[j + 1] {
                    a.swap(j, j+1);
                    swapped = true;
                }
            }
        }
    }
}

fn main() {
    println!("Bubble sort");
    let mut arr = [11,3,7,9,10,12,14];
    bubble_sort(&mut arr);
    print!("{:?}", arr);
}

