#[cfg(not(target_os = "linux"))]
fn main() -> anyhow::Result<()> {
    fittrack::app::run()
}

#[cfg(target_os = "linux")]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("desktop app is not enabled for Linux builds")
}
