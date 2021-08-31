use super::*;

use chrono::naive::NaiveDateTime;
use tabled::Column;

#[derive(Tabled)]
struct SnapshotTableEntry<'a> {
    #[header("ID")]
    id: &'a str,

    #[header("Name")]
    name: &'a str,

    #[header("Size")]
    size: String,

    #[header("Time")]
    time: NaiveDateTime,
}

pub fn output_info(qcow: &qcow::Qcow2) {
    macro_rules! bold {
        () => { Format(|text| text.bold().to_string()) }
    }
    let qcow_table = Table::new([
        ("Version:", qcow.header.version.to_string()),
        ("Backing File:", qcow.header.backing_file.clone().unwrap_or_else(|| "None".into())),
        ("Size:", qcow.header.size.file_size(opts::BINARY).unwrap_or_else(|x| x)),
        ("Cluster Size:", format!("0x{:x}", qcow.cluster_size())),
        ("Encryption:", match qcow.header.crypt_method {
            qcow::EncryptionMethod::Aes => "AES",
            qcow::EncryptionMethod::Luks => "LUKS",
            qcow::EncryptionMethod::None => "None",
        }.to_owned()),
        ("Compression:", match qcow.header.v3_header
            .as_ref()
            .map(|hdr| hdr.compression_type)
            .unwrap_or_default()
        {
            qcow::CompressionType::Zlib => "zlib",
            qcow::CompressionType::Zstd => "zstandard",
        }.to_owned()),
    ])
    .with(Modify::new(Column(..=0)).with(bold!()))
    .with(Style::noborder())
    .with(Modify::new(Full).with(Alignment::left()))
    .with(Disable::Row(..1));

    if qcow.snapshots.is_empty() {
        println!(
            "{}",
            Table::new([
                qcow_table.to_string(),
                Table::new(["No snapshots present"])
                    .with(Header("QCOW Snapshots"))
                    .with(Disable::Row(1..=1))
                    .with(Style::pseudo())
                    .with(
                        Modify::new(Head)
                            .with(Alignment::center_horizontal())
                            .with(bold!())
                    )
                    .to_string()
            ])
            .with(Disable::Row(0..=0))
            .with(Modify::new(Full).with(Indent::new(3, 3, 1, 0,)))
            .with(Style::noborder())
        )
    } else {
        let table: Vec<_> = qcow
            .snapshots
            .iter()
            .map(|snapshot| SnapshotTableEntry {
                id: &snapshot.unique_id,
                name: &snapshot.name,
                size: snapshot
                    .vm_state_size
                    .file_size(opts::BINARY)
                    .unwrap_or_else(|x| x),
                time: NaiveDateTime::from_timestamp(
                    snapshot.time.secs as _,
                    snapshot.time.nanosecs as _,
                ),
            })
            .collect();

        let table = Table::new(&table)
            .with(Header("QCOW Snapshots"))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(..=1)).with(bold!()))
            .with(Style::pseudo())
            .with(
                Modify::new(Row(2..))
                    .with(Alignment::left())
                    .with(Indent::new(1, 1, 0, 0)),
            );

        println!(
            "\n{}",
            Table::new([qcow_table.to_string(), table.to_string()])
                .with(Disable::Row(0..=0))
                .with(Modify::new(Full).with(Indent::new(3, 3, 0, 0,)))
                .with(Style::noborder())
        );
    }
}
