use std::collections::BTreeMap;

// Done at like 3 am, not gonna optimize sorry

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    if l.is_empty() {
        return vec![];
    }
    let mut list =l.iter().map(|x| x.abs()).collect::<Vec<i64>>();
    list.sort();
    let mut multiple_storage: BTreeMap<i64, i64> = BTreeMap::new();
    let primes_list = primes(*list.last().unwrap());
    // println!("{:?}", primes_list);   
    // println!("{:?}", multiple_storage);
    for prime in primes_list.iter() {
        let multiple_of_prime_list: Vec<i64> = l
        .iter()
        .filter_map(|multiple| if *multiple % prime == 0{
            Some(*multiple)
        } else {
            Some(0)
        }).collect();
        // println!("{:?}", multiple_of_prime_list);
        if !do_vecs_match(&multiple_of_prime_list, &[0].repeat(l.len())) {
        let sum_of_multiple:i64 = multiple_of_prime_list.iter().sum();

        
        // println!("{:?}", sum_of_multiple);
        
        multiple_storage.insert(*prime, sum_of_multiple);
        }
    }
    // println!("{:?}", multiple_storage);
    let mut returned_vec: Vec<(i64, i64)> =  Vec::new();
    // let primes_list_clone = primes(*list.last().unwrap());
    for (key, value) in multiple_storage.iter() {
        // let value = match primes_list_clone.get(*key as usize) {
        //     Some(value) => value,
        //     None => &0,
        // };
        // if *value != 0 { 
        let return_tuple: (i64, i64) = (*key, *value);
        returned_vec.push(return_tuple);
    // }
    }
    return returned_vec;
    
}

fn primes(max: i64) -> Vec<i64> {
    let mut primes = vec![2];
    for n in 2..max+1 {
        if primes.iter().all(|p| n % p != 0) {primes.push(n);}
    }
    primes
}

fn do_vecs_match(a: &Vec<i64>, b: &Vec<i64>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
