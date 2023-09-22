use conf_parser::processer;
use std::fs::File;

fn main() {
    let contents = File::open("./test_files/sample_lnd.conf").expect("file read error");
    let lnd_conf = processer::read_to_file_conf(&contents).unwrap();
    processer::write_to_file(&lnd_conf, "./test_files/lnd_1.conf").unwrap();

    let contents = File::open("./test_files/sample_bitcoind.conf").expect("file read error");
    let bitcoind_conf = processer::read_to_file_conf(&contents).unwrap();
    processer::write_to_file(&bitcoind_conf, "./test_files/bitcoind_1.conf").unwrap();

    let contents = File::open("./test_files/sample_lnd_2.conf").expect("file read error");
    let lnd_conf = processer::read_to_file_conf(&contents).unwrap();
    processer::write_to_file(&lnd_conf, "./test_files/lnd_2.conf").unwrap();

    let contents = File::open("./test_files/sample_bitcoind_2.conf").expect("file read error");
    let bitcoind_conf = processer::read_to_file_conf(&contents).unwrap();
    processer::write_to_file(&bitcoind_conf, "./test_files/bitcoind_2.conf").unwrap();

    let contents = File::open("./test_files/sample_cln.conf").expect("file read error");
    let bitcoind_conf = processer::read_to_file_conf(&contents).unwrap();
    processer::write_to_file(&bitcoind_conf, "./test_files/cln_1.conf").unwrap();
}
