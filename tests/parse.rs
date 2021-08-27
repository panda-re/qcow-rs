
//const TEST_PATH: &str = "/home/jamcleod/.panda/bionic-server-cloudimg-amd64-noaslr-nokaslr.qcow2";
const TEST_PATH: &str = "/home/jamcleod/.panda/bionic-shrunk.qcow2";
//const TEST_PATH: &str = "/home/jamcleod/.panda/debian_7.3_arm.qcow";
//const TEST_PATH: &str = "/home/jamcleod/.panda/bionic_v1.qcow";

#[test]
fn parse() {
    use std::io::Read;
    use std::io::Seek;

    let qcow = qcow::open(TEST_PATH).unwrap();

    for snapshot in qcow.snapshots() {
        println!(
            "Snapshot {:?}: {:?} (size = {})",
            snapshot.unique_id,
            snapshot.name,
            snapshot.vm_state_size
        );
    }

    let qcow = qcow.unwrap_qcow2();
    dbg!(&qcow);
    let mut file = std::io::BufReader::new(std::fs::File::open(TEST_PATH).unwrap());
    let _ = qcow
            .l1_table[0]
            .read_l2(&mut file, qcow.header.cluster_bits);

    let mut file = std::fs::File::open(TEST_PATH).unwrap();
    let mut reader = qcow.reader(&mut file);

    let mut buf = [0; 0x100];
    reader.read_exact(&mut buf).unwrap();

    let mut buf2 = [0; 0x80];
    let pos = reader.seek(std::io::SeekFrom::Current(-0x80)).unwrap();
    assert_eq!(pos, 0x80);
    reader.read_exact(&mut buf2).unwrap();
    assert_eq!(buf2, &buf[0x80..]);

    let pos = reader.seek(std::io::SeekFrom::Start(0)).unwrap();
    assert_eq!(pos, 0);
    reader.read_exact(&mut buf2).unwrap();
    assert_eq!(buf2, &buf[..0x80]);

    std::io::copy(
        &mut reader.take(0x400_0000),
        &mut std::io::sink(),
    ).unwrap();
}
