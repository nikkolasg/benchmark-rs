use anyhow::Result;
use std::{fs::File, path::PathBuf};
use tracing::info;

/// Structure that records each operation within `bench` method and writes the output to a CSV
/// file.
#[derive(Clone, Debug)]
pub struct Benchmarker {
    csv_path: PathBuf,
    prefix: String,
}

const DEFAULT_BENCH_FILE: &str = "bench.csv";

impl Benchmarker {
    pub fn new_from_path(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            // only write the header if the file doesn't exists
            let writer = File::options().create(true).append(true).open(&path)?;
            let mut wtr = csv::Writer::from_writer(writer);
            wtr.write_record(["name", "time"])?;
        }
        info!("Benchmarker setup to write output in {:?}", path);
        Ok(Self {
            prefix: "".to_string(),
            csv_path: path,
        })
    }

    pub fn with_prefix(&self, prefix: &str) -> Self {
        let mut new = self.clone();
        new.prefix = prefix.to_string();
        new
    }

    pub fn bench<F, O>(&self, name: &str, f: F) -> Result<O>
    where
        F: FnOnce() -> Result<O>,
    {
        let now = std::time::Instant::now();
        let output = f()?;
        let elapsed = now.elapsed().as_millis();
        self.write_to_csv(name, elapsed)?;
        Ok(output)
    }

    pub fn write_to_csv(&self, name: &str, elapsed: u128) -> Result<()> {
        let writer = File::options().append(true).open(&self.csv_path)?;
        let mut wtr = csv::Writer::from_writer(writer);
        let final_name = if self.prefix.is_empty() {
            name.to_string()
        } else {
            self.prefix.clone() + "-" + name
        };
        wtr.write_record([final_name, elapsed.to_string()])?;
        wtr.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::Benchmarker;
    use anyhow::Result;
    #[test]
    fn benchmarker() -> Result<()> {
        let path = testfile::generate_name();
        let b = Benchmarker::new_from_path(path.clone())?;
        b.bench("test_fun", || {
            let _total: u32 = (0..10000).sum();
            Ok(())
        })?;
        assert!(Path::new(&path).exists());
        Ok(())
    }
}
