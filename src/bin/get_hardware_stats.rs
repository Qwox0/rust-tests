fn main() -> Result<(), anyhow::Error> {
    macro_rules! out {
        ($val:expr) => {
            println!("{}: {:?}", stringify!($val), $val);
        };
    }

    println!("\n# Cpus:");
    out!(std::thread::available_parallelism()?);
    out!(num_cpus::get());
    out!(num_cpus::get_physical());

    println!("\n# L1 Cache:");
    out!(cache_size::l1_cache_size());
    out!(cache_size::l1_cache_line_size());

    println!("\n# L2 Cache:");
    out!(cache_size::l2_cache_size());
    out!(cache_size::l2_cache_line_size());

    println!("\n# L3 Cache:");
    out!(cache_size::l3_cache_size());
    out!(cache_size::l3_cache_line_size());

    Ok(())
}
