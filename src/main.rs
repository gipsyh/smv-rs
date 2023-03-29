use smv::SMV;

fn main() {
    // let smv = SMV::from_file("./examples/counter-flat.smv").unwrap();
    let smv = SMV::from_file("./examples/abp8.smv").unwrap();
    // let smv =
    //     SMV::from_file("../MC-Benchmark/NuSMV-2.6-examples/example_cmu/dme1-flat.smv").unwrap();
    // dbg!(&smv);
    for ltl in smv.ltlspecs.iter() {
        println!("{}", ltl);
    }
}
