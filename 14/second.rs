use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut quality: HashMap<String, i64> = HashMap::new();
    let mut bom: HashMap<String, HashMap<String, i64>> = HashMap::new();
    let mut ingress: HashMap<String, HashSet<String>> = HashMap::new();

    let content = std::fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        let eq: Vec<&str> = line.split("=>").collect();
        let target: Vec<&str> = eq[1].split_whitespace().collect();

        quality.insert(String::from(target[1]),
            target[0].parse::<i64>().unwrap());

        let mut elems: HashMap<String, i64> = HashMap::new();
        for part in eq[0].split(",") {
            let d: Vec<&str> = part.split_whitespace().collect();
            elems.insert(String::from(d[1]), d[0].parse::<i64>().unwrap());

            let ing = ingress.entry(d[1].to_string())
                .or_insert(HashSet::<String>::new());
            ing.insert(String::from(target[1]));
        }
        bom.insert(String::from(target[1]), elems);
    }

    let mut nc : i64 = 2073023; 
    loop {
        let mut ningress = ingress.clone();

        let mut want: HashMap<String, i64> = HashMap::new();
        let mut pending: HashSet<String> = HashSet::new();
        want.insert("FUEL".to_string(), nc); 
        pending.insert("FUEL".to_string());
        let mut count :i64 = 0;
 
        while !pending.is_empty() {
            let elem = pending.iter().next().unwrap().to_string();
            
            let needed = want.get(&elem).unwrap();
            let base = quality.get(&elem).unwrap();
            let mut factor: i64 = (*needed) / (*base); 
            if (*needed) % (*base) != 0 {
                factor += 1;
            }

            for (nelem, nbase) in bom.get(&elem.to_string()).unwrap().iter() {
                if nelem == "ORE" {
                    count += nbase * factor;
                    continue;
                }
 
                let ings = ningress.get_mut(nelem).unwrap();
                ings.remove(&elem.to_string());
                if ings.is_empty() {
                    pending.insert(nelem.to_string());
                }
 
                let amount = want.entry(nelem.to_string()).or_insert(0);
                *amount += nbase * factor;
            }

            pending.remove(&elem);
        }
        if count > 1000000000000 {
            break;
        }
        nc += 1;
        println!("current {} {}", nc, count);    
    }

    println!("{}", nc - 1);
}
