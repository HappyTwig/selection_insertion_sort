use rand::Rng;

fn main() {
    println!("random number!");

    const OCCUR_NUM: usize = 100;

    // -1000以上1000以下のランダムな整数を100回発生、表示
    let mut array = [0;OCCUR_NUM];
    for i in 0..OCCUR_NUM {
        let random_num = rand::thread_rng().gen_range(-1000,1001);
        array[i] = random_num;
        println!("array[{}]:{}", i, array[i]);
    }

    // 選択ソートで昇順に並び替え
    let mut selection_array = array.clone();
    let mut min_index;
    for j in 0..OCCUR_NUM {
        min_index = j;
        for i in j+1..OCCUR_NUM {
            if selection_array[min_index] > selection_array[i] {
                min_index = i;
            }
        }
        let min;
        min = selection_array[min_index];
        selection_array[min_index] = selection_array[j];
        selection_array[j] = min;
    }
    println!("selection number!");
    for i in 0..OCCUR_NUM {
        println!("selection_array[{}]:{}", i, selection_array[i]);
    }

    // 挿入ソートで昇順に並べ替え
    let mut insertion_array = array.clone();
    for j in 1..OCCUR_NUM {
        for i in 0..j {
            if insertion_array[j] < insertion_array[i] {
                let ins_num = insertion_array[j];
                let mut k = j;
                while k > i {
                    insertion_array[k] = insertion_array[k-1];
                    k = k-1;
                }
                insertion_array[i] = ins_num;
                break;
            }
        }
    }
    println!("insertion number!");
    for i in 0..OCCUR_NUM {
        println!("insertion_array[{}]:{}", i, insertion_array[i]);
    }
}