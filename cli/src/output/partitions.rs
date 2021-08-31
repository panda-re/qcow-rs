use super::*;

#[derive(Tabled)]
struct TableEntry<'a> {
    #[header("ID")]
    id: usize,

    #[header("Size")]
    size: String,

    #[header("OS")]
    os: &'a str,

    #[header("Partition Contents")]
    type_description: &'a str,
}

pub fn output_partitions(reader: &mut (impl Read + Seek)) {
    let partitions = bootsector::list_partitions(reader, &Default::default()).unwrap();

    let table: Vec<_> = partitions
        .iter()
        .map(|part| {
            let guid_info = match &part.attributes {
                Attributes::GPT { type_uuid, .. } => PartitionTypeGuid::from_bytes(*type_uuid)
                    .description()
                    .unwrap_or(PartitionDescription {
                        os: "Unknown",
                        type_description: "Unknown",
                    }),
                Attributes::MBR { .. } => panic!("MBR is unsupported"),
            };

            TableEntry {
                id: part.id,
                size: part.len.file_size(opts::BINARY).unwrap_or_else(|x| x),
                os: guid_info.os,
                type_description: guid_info.type_description,
            }
        })
        .collect();

    let table = Table::new(&table)
        .with(Header("QCOW Partitions"))
        .with(Modify::new(Head).with(Alignment::center_horizontal()))
        .with(Modify::new(Row(..=1)).with(Format(|text| text.bold().to_string())))
        .with(Style::pseudo())
        .with(
            Modify::new(Row(2..))
                .with(Alignment::left())
                .with(Indent::new(1, 1, 0, 0)),
        );

    println!(
        "\n{}",
        Table::new([table.to_string()])
            .with(Disable::Row(0..=0))
            .with(Modify::new(Full).with(Indent::new(3, 3, 1, 0,)))
            .with(Style::noborder())
    );
}
