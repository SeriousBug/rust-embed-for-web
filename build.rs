use std::env;
use std::fs;
use std::path::PathBuf;

/// Generates a large fixture folder for benchmarks, gated behind the
/// `bench-fixtures` feature. When the feature is off this is a no-op, so
/// normal and published builds are unaffected.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=BENCH_FIXTURE_COUNT");

    // Only generate fixtures when the bench-fixtures feature is enabled.
    if env::var_os("CARGO_FEATURE_BENCH_FIXTURES").is_none() {
        return;
    }

    let count: usize = env::var("BENCH_FIXTURE_COUNT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo"));
    let fixture_dir = out_dir.join(format!("bench_fixtures_{count}"));

    // Idempotent: only (re)generate when the file count does not match.
    let needs_generation = match fs::read_dir(&fixture_dir) {
        Ok(entries) => entries.count() != count,
        Err(_) => true,
    };

    if needs_generation {
        // Start from a clean directory so a stale partial run can't leave junk.
        let _ = fs::remove_dir_all(&fixture_dir);
        fs::create_dir_all(&fixture_dir).expect("create fixture dir");
        for i in 0..count {
            let path = fixture_dir.join(format!("file_{i:05}.txt"));
            // ~100 bytes of deterministic content per file.
            let body = format!(
                "fixture file number {i:05} -- padding 0123456789012345678901234567890123456789012345\n"
            );
            fs::write(&path, body).expect("write fixture file");
        }
    }

    let abs = fixture_dir
        .canonicalize()
        .expect("canonicalize fixture dir");
    println!("cargo:rustc-env=BENCH_FIXTURE_DIR={}", abs.display());
}
