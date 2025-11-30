use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi", "Osazuwa Faith Etiyeye"];
    let ministries = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zones = vec!["South West","North East","South South","South West","South East"];


    let mut file = File::create("efcc_merged.txt").expect("Could not create file");
    file.write_all("EFCC MERGED CONVICTED MINISTERS DATA\n\n".as_bytes()).expect("Cannot write to file");
    file.write_all("S/N | COMMISSIONER | MINISTRY | GEO-POLITICAL ZONE\n".as_bytes()).expect("Cannot write to file");

    for i in 0..commissioners.len() {
        let list = format!(
            "{} | {} | {} | {}\n",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );

        file.write_all(list.as_bytes()).expect("Cannot write to file");
    }

    println!("File 'efcc_merged.txt' created successfully");
}
