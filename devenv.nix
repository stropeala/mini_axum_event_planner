{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  languages.rust = {
    enable = true;
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  env = {
    DATA_DIR_PATH = "${config.devenv.root}/data";
    EVENT_PLANNER_SCHEMA = "${config.devenv.root}/data/event_planner_schema.sql";
    USER_SCHEMA = "${config.devenv.root}/data/user_schema.sql";
  };
}
