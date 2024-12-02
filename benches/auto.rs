use autorenamer::file_ops::{get_episodes, rename_episodes};
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_rename(bencher: divan::Bencher) {
    let base_path = "./test_files/";
    create_dir_all(base_path).expect("Failed to create directory");
    for number in 1..=50000 {
        let filename = format!("{}Episode {:05}.mp4", base_path, number);
        let mut file = File::create(&filename).expect("Failed to create file");
        file.write_all(b"").expect("Failed to write to file");
    }
    bencher.bench(|| {
        let episodes = get_episodes(base_path.to_string());
        match episodes {
            Ok(result) => {
                rename_episodes(result, 69, base_path.to_string(), 0, false, false);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    });
    remove_dir_all(base_path).expect("Failed to remove directory");
}
