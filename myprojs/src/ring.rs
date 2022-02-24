const MAX_NUM: usize = 200;

pub fn ring() {
    let n = 41;
    let m = 3;

    let mut flag: [usize; MAX_NUM] = [0; MAX_NUM];

    let mut count = 0;
    let mut num = 0;

    (1..=n).for_each(|i| flag[i] = 1);

    while count < n - 1 {
        for i in 1..=n {
            println!("当前位置 {}, 报数 j = {}", i, num + 1);
            if 1 == flag[i] {
                num += 1;

                if num == m {
                    println!("{} --> out", i);
                    count += 1;
                    flag[i] = 0;
                    num = 0;
                }

                if count == n - 1 {
                    break;
                }
            }
        }
    }

    for i in 1..=n {
        if 1 == flag[i] {
            println!("{} --> out", i);
            println!("Josephus ring game is over!");
        }
    }
}
