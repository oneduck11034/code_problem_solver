// https://velog.io/@aoleejohn/C-%ED%94%84%EB%A1%9C%EA%B7%B8%EB%9E%98%EB%A8%B8%EC%8A%A4-%EB%8B%AC%EB%A6%AC%EA%B8%B0-%EA%B2%BD%EC%A3%BC
use std::collections::HashMap;

fn main() {
    // input
    let mut players: Vec<&str>= Vec::new();
    let mut callings: Vec<&str>= Vec::new();

    // forget enumerate
    let mut result_v: Vec<&str>= Vec::new();
    let mut hashmap_a: HashMap<usize, &str>= HashMap::new();
    let mut hashmap_b: HashMap<&str, usize>= HashMap::new();

    let mut i=0_usize;
    for _ in 0..players.len(){
        // hashmap_a[i]= players[i];
        hashmap_a.insert(i, players[i]);
        // hashmap_b[players[i]]= i;
        hashmap_b.insert(players[i], i);
    }

    let mut i= 0_usize;
    for e in &callings{
        let mut cur_idx = hashmap_b[callings[i]];
        let pre_person= cur_idx - 1_usize;
        let change= hashmap_a[&pre_person];
        // hashmap_a[&pre_person]= callings[i];
        hashmap_a.insert(pre_person, callings[i]);
        // hashmap_a[&cur_idx] = change;
        hashmap_a.insert(cur_idx, change);

        // hashmap_b[callings[i]] = pre_person;
        hashmap_b.insert(callings[i], pre_person);
        // hashmap_b[change] = cur_idx;
        hashmap_b.insert(change, cur_idx);
    }

    for e in hashmap_a {
        result_v.push(e.1);
    }

    println!("{:?}", result_v);
}
