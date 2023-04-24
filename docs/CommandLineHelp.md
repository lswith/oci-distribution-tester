# Command-Line Help for `oci-tester`

This document contains the help content for the `oci-tester` command-line program.

**Command Overview:**

* [`oci-tester`↴](#oci-tester)
* [`oci-tester push-images`↴](#oci-tester-push-images)
* [`oci-tester pull-images`↴](#oci-tester-pull-images)
* [`oci-tester push-image-list`↴](#oci-tester-push-image-list)

## `oci-tester`

A tool for testing OCI distribution servers

**Usage:** `oci-tester [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `push-images` — Pushes a generated OCI image to an OCI distribution server
* `pull-images` — Pulls OCI images from an OCI distribution server
* `push-image-list` — 

###### **Options:**

* `-v`, `--verbose`



## `oci-tester push-images`

Pushes a generated OCI image to an OCI distribution server

**Usage:** `oci-tester push-images [OPTIONS]`

###### **Options:**

* `--reg-url <REGISTRY_URL>` — The OCI distribution server url

  Default value: `http://localhost:6000`
* `--reg-userpass <REGISTRY_USERPASS>` — The user+password to authenticate against the OCI distribution server in the format user:password
* `-c`, `--count <COUNT>` — The amount of images to push

  Default value: `1`



## `oci-tester pull-images`

Pulls OCI images from an OCI distribution server

**Usage:** `oci-tester pull-images [OPTIONS]`

###### **Options:**

* `--reg-url <REGISTRY_URL>` — The OCI distribution server url

  Default value: `https://index.docker.io`
* `--reg-userpass <REGISTRY_USERPASS>` — The user+password to authenticate against the OCI distribution server in the format user:password
* `-i`, `--image <IMAGE>` — The image to pull

  Default value: `alpine:latest`
* `-c`, `--count <COUNT>` — The amount of images to pull

  Default value: `1`



## `oci-tester push-image-list`

**Usage:** `oci-tester push-image-list [OPTIONS]`

###### **Options:**

* `--reg-url <REGISTRY_URL>` — The OCI distribution server url

  Default value: `http://localhost:6000`
* `--reg-userpass <REGISTRY_USERPASS>` — The user+password to authenticate against the OCI distribution server in the format user:password
* `-i`, `--image <IMAGE>` — Where to push the image list

  Default value: `test/this:cache`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

