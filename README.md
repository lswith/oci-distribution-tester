# OCI Tester

oci-tester is a command-line tool for testing the functionality
of an OCI Distribution Server, which is essentially a Docker registry
that conforms to the OCI spec. The tool supports both pushing and pulling
images to and from a configured distribution server, and can be useful for
verifying that your registry is working correctly.

## Installation

You can clone this repo and run `cargo install`:

```
git clone git@github.com:lswith/oci-tester.git
cd oci-tester/
cargo install --path .
```

You can also download your system's binary from the releases section:

[Releases](https://github.com/lswith/oci-tester/releases).

## Usage

To push a generated image to your local registry (localhost:6000):

```
oci-tester push-images
```

You can also pull images from a registry (docker.io):

```
oci-tester pull-images
```

For more detailed information on the available subcommands and their options, see [oci-tester.md](./docs/CommandLineHelp.md).

## License

This tool is licensed under the [MIT License](./LICENSE).
