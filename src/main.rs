use serde::{Deserialize, Serialize};
use std::fs;
use crate::fs::File;
use std::io::BufReader;
use std::io::Read;
use bincode;
use std::io;
pub mod mp;


fn main() {

let path1 = "data/interactiondata.json".to_owned();
    let cont1 = fs::read_to_string(path1);

let path2 = "data/factordata.json".to_owned();
    let cont2 = fs::read_to_string(path2);

let path3 = "data/render.json".to_owned();
    let cont3 = fs::read_to_string(path3);

let mut cyclelength= i64::new();
        
io::stdin()
    .read_line(&mut cyclelength)
    .expect("Failed to read line");
        
println!("Начинается симуляция на {cyclelength} симуляций");



parser::file1(&cont1);
parser::file2(&cont2);
parser::file3(&cont3);


#[derive(Debug, Deserialize)]
struct interactiondata {
    faclist: Vec<String>,
    fmodval: Hashmap<String, String>,
    emodval: Hashmap<String, String>
}

#[derive(Debug, Deserialize)]
struct factordata {
    factorname: String,
    facvalue: f64,
    expvalue: f64
}

#[derive(Debug, Deserialize)]
struct render {
    x:Hashmap<factname, factval, exprval>,
    y:Hashmap<factname, factval, exprval>
}
////
struct factname(String, String);

struct factval(String, Vec<f64>);

struct exprval(String, Vec<f64>);

///
///// создать стракты которые будут использщованы как итераторы

let i: interactiondata = serde_json::from_str(cont1)?;
let f: factordata = serde_json::from_str(cont2)?;
let r: render = serde_json::from_str(cont3)?;

fn createfile(i: i64) {
    let mut file = File::create("цикл_{i}.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;


}

fn upd(facname: String, facnamein: String, facval: f64, optype:String) {
    match facnamein {
        facname => facval = mp::wemathin(optype),
        _ => facval = facval
    }
}
let plch1 =r.x.factname;
let plch2 =r.x.factname;
let plch3 =i.faclist;
let plch4 =f.factorname;
let plch5 = f.facvalue;
let plch6 = f.expvalue;
let mut xfname = |plch1| plch1;
let mut yfname = |plch2| plch2;

for i in cyclelength{
    createfile(i);
for x in r{

let mut localfval =     
    (match f.factorname {
        xfname => r.facval,
        yfname => r.facval,
        _ => f.facvalue
    }
    );
    let mut localeval =     
    (match f.factorname {
        xfname => r.exprval,
        yfname => r.exprval,
        _ => f.expvalue
    }
    );
for y in r{

for factorname in f {
    let localfname = |plch4| plch4;

        for faclist in f {
            let plc1 = plch3;
            let plc2 = plch4;
            let dmofv = i.fmodval;
            let mut tempf = plch5 ; 
            let mut tempe = plch6 ;
        
            fn outputf() {
            
            match |plch3| plch3 {
                localfname =>upd(|plc1| plc1,|plc2| plc2,|localfval| localfval,|fmodv| fmodv),
                _ =>|tempf| tempf
            }
            }
            fn outpute() {
            match |plch3| plch3 {
                localfname => upd(|plc1| plc1,|plc2| plc2,|localeval| localeval,|fmodv| fmodv),
                _ => |tempe|tempe
            }
        }
        }
    }
    let outputvec1 = vec![];
    outputvec1.push(outputf);


}    
assert_eq!(outputvec1);
}
}
}
