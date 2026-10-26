{
  installShellFiles,
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "umbrella-fetch";
  version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
      ./assets
      ./LICENSE
    ];
  };
  cargoLock.lockFile = ./Cargo.lock;
  nativeBuildInputs = [ installShellFiles ];
  postInstall = ''
    for shell in bash zsh fish; do
      installShellCompletion --cmd umbrella-fetch --$shell <($out/bin/umbrella-fetch completions $shell)
    done
    install -Dm644 LICENSE "$out/share/doc/umbrella-fetch/LICENSE"
  '';
  meta = {
    homepage = "https://github.com/ezequielgk/Umbrella-Fetch";
    description = "Interactive TUI dashboard inspired by Resident Evil";
    mainProgram = "umbrella-fetch";
    license = lib.licenses.bsd3;
    platforms = lib.platforms.unix;
  };
}
