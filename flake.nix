{
  inputs.nix-cargo-integration = {
    type = "github";
    owner = "yusdacra";
    repo = "nix-cargo-integration";
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    nix-cargo-integration,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [nix-cargo-integration.flakeModule];
      perSystem = {
        config,
        pkgs,
        lib,
        ...
      }: let
        # shorthand for accessing outputs
        # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
        outputs = config.nci.outputs;
        extraPackages = with pkgs; [elf2uf2-rs rust-analyzer];
      in {
        # declare projects
        nci.projects.morse = {
          path = ./.;
          # export all crates (packages and devshell) in flake outputs
          # alternatively you can access the outputs and export them yourself
          export = true;
          # targets.thumbv6m-none-eabi.default = true;
        };
        # nci.crates.pico.targets.thumbv6m-none-eabi.default = true;
        nci.toolchainConfig = (lib.importTOML ./toolchain.toml).toolchain;
        # export the project devshell as the default devshell
        devShells.default = outputs.morse.devShell.overrideAttrs (old: {packages = (old.packges or []) ++ extraPackages;});
      };
    };
}
