use conf_parser::processer;
use std::fs::File;

fn main() {
    let contents = File::open("./test_files/sample_lnd.conf").expect("file read error");
    let lnd_conf = processer::read_to_file_conf(&contents).unwrap();
    /*let _ = processer::write_to_file(&lnd_conf, "./test_files/lnd_2.conf").unwrap();

        let contents = fs::read_to_string("./test_files/sample_bitcoind.conf").expect("file read error");
        let bitcoind_conf = processer::read_to_file_conf(&contents).unwrap();
        let _ = processer::write_to_file(&bitcoind_conf, "./test_files/bitcoind_2.conf").unwrap();
    */
}
