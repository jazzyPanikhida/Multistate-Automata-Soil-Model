pub fn wemathin(_: optype) {   
    let e = optype;
    let ops = ['+', '-', '*', '/', '^'];
    let values: Vec<f64> = e.split(&ops).map(|v| v.trim().parse().unwrap()).collect();
    let operands: Vec<_> = e.matches(&ops).collect();
    
    let (&(mut curr), values) = values.split_first().unwrap();
    for (op, &value) in operands.into_iter().zip(values) {
        match op {
            "+" => { curr = curr + value },
            "-" => { curr = curr - value },
            "*" => { curr = curr * value },
            "/" => { curr = curr / value },
            "^" => { curr = pow(curr, value)},
            _ => unreachable!(),
        }
    }
}
