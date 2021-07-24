use crate::common::*;

pub fn run() -> Result<(), i32> {
  #[cfg(windows)]
  ansi_term::enable_ansi_support().ok();

  env_logger::Builder::from_env(
    env_logger::Env::new()
      .filter("JUST_LOG")
      .write_style("JUST_LOG_STYLE"),
  )
  .init();

  let app = Config::app();

  info!("Parsing command line arguments…");
  let matches = app.get_matches();

  let loader = Loader::new();

  let mut color = Color::auto();
  let mut verbosity = Verbosity::default();

  Config::from_matches(&matches)
    .map_err(JustError::Config)
    .and_then(|config| {
      color = config.color;
      verbosity = config.verbosity;
      config.run_subcommand(&loader)
    })
    .map_err(|error| {
      if let JustError::Code(_) = error {
      } else {
        if !verbosity.quiet() {
          if color.stderr().active() {
            eprintln!("{}: {:#}", color.stderr().error().paint("error"), error);
          } else {
            eprintln!("error: {}", error);
          }
        }
      }

      error.code()
    })
}
