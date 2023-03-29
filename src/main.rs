use smv::SMV;

fn main() {
    // let smv = SMV::from_file("./examples/counter-flat.smv").unwrap();
    let smv =
        SMV::from_file("../MC-Benchmark/NuSMV-2.6-examples/example_cmu/dme1-flat.smv").unwrap();
    dbg!(&smv);
    println!("{}", smv.ltlspecs[0]);
    println!("{}", smv.ltlspecs[1]);
}
