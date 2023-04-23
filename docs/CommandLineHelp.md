# Command-Line Help for `oci-distribution-tester`

This document contains the help content for the `oci-distribution-tester` command-line program.

**Command Overview:**

* [`oci-distribution-tester`↴](#oci-distribution-tester)
* [`oci-distribution-tester push-images`↴](#oci-distribution-tester-push-images)
* [`oci-distribution-tester pull-images`↴](#oci-distribution-tester-pull-images)

## `oci-distribution-tester`

A tool for testing OCI distribution servers

**Usage:** `oci-distribution-tester [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `push-images` — Pushes a generated OCI to a distribution server
* `pull-images` — Pulls OCIs from a distribution server

###### **Options:**

* `--markdown-help`
* `-v`, `--verbose`



## `oci-distribution-tester push-images`

Pushes a generated OCI to a distribution server

**Usage:** `oci-distribution-tester push-images [REGISTRY_URL] [COUNT] [REGISTRY_USERPASS]`

###### **Arguments:**

* `<REGISTRY_URL>` — The distribution server url

  Default value: `http://localhost:6000`
* `<COUNT>` — The amount of images to push

  Default value: `1`
* `<REGISTRY_USERPASS>` — The user+password to authenticate against the distribution server in the format user:password



## `oci-distribution-tester pull-images`

Pulls OCIs from a distribution server

**Usage:** `oci-distribution-tester pull-images [REGISTRY_URL] [COUNT] [REGISTRY_USERPASS] [IMAGE]`

###### **Arguments:**

* `<REGISTRY_URL>` — The distribution server url

  Default value: `https://index.docker.io`
* `<COUNT>` — The amount of images to pull

  Default value: `1`
* `<REGISTRY_USERPASS>` — The user+password to authenticate against the distribution server in the format user:password
* `<IMAGE>` — The image to pull

  Default value: `alpine:latest`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

