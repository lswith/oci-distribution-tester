# Command-Line Help for `oci-tester`

This document contains the help content for the `oci-tester` command-line program.

**Command Overview:**

- [`oci-tester`↴](#oci-tester)
- [`oci-tester push-images`↴](#oci-tester-push-images)
- [`oci-tester pull-images`↴](#oci-tester-pull-images)

## `oci-tester`

A tool for testing OCI distribution servers

**Usage:** `oci-tester [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `push-images` — Pushes a generated OCI image to an OCI distribution server
- `pull-images` — Pulls OCI images from an OCI distribution server

###### **Options:**

- `-v`, `--verbose`

## `oci-tester push-images`

Pushes a generated OCI image to an OCI distribution server

**Usage:** `oci-tester push-images [REGISTRY_URL] [COUNT] [REGISTRY_USERPASS]`

###### **Arguments:**

- `<REGISTRY_URL>` — The OCI distribution server url

  Default value: `http://localhost:6000`

- `<COUNT>` — The amount of images to push

  Default value: `1`

- `<REGISTRY_USERPASS>` — The user+password to authenticate against the OCI distribution server in the format user:password

## `oci-tester pull-images`

Pulls OCI images from an OCI distribution server

**Usage:** `oci-tester pull-images [REGISTRY_URL] [COUNT] [REGISTRY_USERPASS] [IMAGE]`

###### **Arguments:**

- `<REGISTRY_URL>` — The OCI distribution server url

  Default value: `https://index.docker.io`

- `<COUNT>` — The amount of images to pull

  Default value: `1`

- `<REGISTRY_USERPASS>` — The user+password to authenticate against the OCI distribution server in the format user:password
- `<IMAGE>` — The image to pull

  Default value: `alpine:latest`

<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
