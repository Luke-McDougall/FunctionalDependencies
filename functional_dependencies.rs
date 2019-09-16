use std::env;

struct FD {
    X: String,
    A: String,
}

fn closures(mut closure_vec: &mut Vec<FD>, relation: &String, start: usize, right: String) {
    if relation.len() - start > 0 {
        let r = relation.as_bytes();
        for i in start..r.len() {
            let next_right = format!("{}{}", right, (r[i] as char).to_string());
            closures(&mut closure_vec, &relation, i + 1, next_right.clone());
            if next_right != *relation {
                closure_vec.push(FD {X: next_right.clone(), A: next_right});
            }
        }
    }
}

fn dependencies(closure_vec: &mut Vec<FD>, base_fd: &mut Vec<FD>) {
    let closure_length = closure_vec.len();
    let base_length = base_fd.len();
    let mut index = 0;

    while index < closure_length {
        let mut found = false;
        'outer: for j in 0..base_length {
            let word = &base_fd[j].X; 
            for c in word.chars() {
                if !closure_vec[index].A.contains(c) {continue 'outer;}
            }
            
            if !closure_vec[index].A.contains(&base_fd[j].A) {
                closure_vec[index].A.push_str(&base_fd[j].A);
                found = true;
            }
        }
        if !found {
            index += 1;
        }
    }
}

fn non_trivial(closure_vec: &Vec<FD>) -> Vec<FD> {
    let mut fd_vec = Vec::new();

    for closure_fd in closure_vec.iter() {
        for c in closure_fd.A.chars() {
            if !closure_fd.X.contains(c) {
                fd_vec.push(FD {X: closure_fd.X.clone(), A: c.to_string()});
            }
        }
    }
    fd_vec
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut base_fd = Vec::new();

    let relation = args[1].clone();
    for e in args.iter().skip(2) {
        let mut X = e.to_string();
        let idx = X.find(",").unwrap();
        let mut A = X.split_off(idx);
        A.remove(0);
        let fd = FD {X, A};
        base_fd.push(fd);
    }

    let mut closure_vec = Vec::new();
    for c in relation.chars() {
        closure_vec.push(FD {X: c.to_string(), A: c.to_string()});
    }

    for (i, e) in relation.chars().enumerate() {
        let right = e.to_string();
        closures(&mut closure_vec, &relation, i + 1, right);
    }

    closure_vec.sort_by_key(|a| a.X.len());

    dependencies(&mut closure_vec, &mut base_fd);
    let fd_vec = non_trivial(&closure_vec);
    for fd in closure_vec.iter() {
        println!("{}+ = {}", fd.X, fd.A);
    }
    println!("Non trivial functional dependencies");
    for s in fd_vec.iter() {
        println!("{}->{}", s.X, s.A);
    }

}
