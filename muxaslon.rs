use std::collections::HashSet;

fn find_variables() {
    let mut solutions = 0;

    for m in 0..10 {
        for u in 0..10 {
            for x in 0..10 {
                for a in 0..10 {
                    for s in 0..10 {
                        for l in 0..10 {
                            for o in 0..10 {
                                for n in 0..10 {
                                    let mut digits = HashSet::new();
                                    digits.insert(m);
                                    digits.insert(u);
                                    digits.insert(x);
                                    digits.insert(a);
                                    digits.insert(s);
                                    digits.insert(l);
                                    digits.insert(o);
                                    digits.insert(n);

                                    if digits.len() == 8 {
                                        if m * 1000 + u * 100 + x * 10 + a == s * 1000 + l * 100 + o * 10 + n {
                                            solutions += 1;

                                            println!("{:>4}{:>4}", format!("{}{}", m, u), format!("{}", a));
                                            println!("    ------");
                                            println!("{:>4}", format!("{}", s));
                                            println!("{}", format!("{}", l));
                                            println!("{}", format!("{}", o));
                                            println!("{}", format!("{}", n));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\nЗагальна кількість рішень: {}", solutions);
}

fn main() {
    find_variables();
}
