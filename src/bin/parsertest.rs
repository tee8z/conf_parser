use std::fs;

fn main() {
    let contents = fs::read_to_string("./test_files/sample_lnd.conf").expect("file read error");
    let contents = fs::read_to_string("./test_files/sample_bitcoind.conf").expect("file read error");

    //let lnd_conf 
}