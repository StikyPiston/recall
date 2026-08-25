# Recall

![recall's terminal output](assets/freeze.png)

**recall** is a minimal to-do list/reminders tool for the CLI!

## Installation

### with Nix

Add `recall` to your flake inputs

```nix
# flake.nix
{
  inputs = {
    # ...
    recall = {
      url = "github:indium114/recall";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

And add the package to `environment.systemPackages`

```nix
{
  pkgs,
  inputs, # make sure to take in `inputs` if you don't have it in specialArgs
  ...
}:

{
  # ...
  environment.systemPackages = [
    inputs.recall.${pkgs.stdenv.hostPlatform.system}.recall
  ];
}
```

### from the Binary

Go to the *Releases* section on the right, click the latest release, and click the binary for your architecture to download it.

> [!note]
> On macOS, you will have to compile `recall` from source.

### with [wares](https://github.com/indium114/wares)

Simply add the following to your `config.yaml`:

```yaml
wares:
  recall:
    name: recall
    repo: indium114/recall
    asset: "recall_Linux_x86_64"
```
> replace `x86_64` with `arm64` if you're on an ARM processor.

### with cargo

Run the following to install *recall*. Ensure that `~/.cargo/bin` is in your `$PATH`

```shell
cargo install --git https://github.com/indium114/recall
```
