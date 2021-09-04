# Bovine

[![release](https://img.shields.io/github/v/release/nickgerace/bovine?sort=semver&style=flat-square&logo=github&color=blue)](https://github.com/nickgerace/bovine/releases/latest)
[![crates.io](https://img.shields.io/crates/v/bovine?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/bovine)
[![build](https://img.shields.io/github/workflow/status/nickgerace/bovine/merge?style=flat-square)](https://github.com/nickgerace/bovine/actions)
[![license](https://img.shields.io/github/license/nickgerace/bovine?style=flat-square&color=purple)](./LICENSE)

Manage single node [Rancher](https://rancher.com/) clusters with a single binary, `bovine`.

```
% bovine run
Pulling [rancher/rancher:latest], this may take awhile...
Rancher container is running: ead7ff0c711a

% bovine list
ead7ff0c711a [rancher/rancher:latest] (running) > Up 5 seconds

% bovine stop --all
Stopped Rancher container: ead7ff0c711a
```

## Background

`bovine` is simultaneously designed to be an accessible entrypoint into using both Rancher (and Kubernetes) and an efficient manager for experienced users working with single node Rancher clusters.

- New to Rancher or Kubernetes?
  - `bovine` aims to be one of the first stepping stones into trying both technologies for the first time.
  - Those familiar with Docker, but unfamiliar with Kubernetes or Rancher, should feel right at home.
  - Creating your first cluster requires no arguments, flags, or setup. Just have Docker running, execute `bovine run`, navigate to your favorite browser, and access `127.0.0.1`.
- Advanced User?
  - Single node Rancher installations are useful for trying out new Rancher releases, provisioning downstream clusters for development, and general lab usage.
  - `bovine` is designed for multi-platform use (no need to maintain both Bash and PowerShell scripts).
  - Bring your own Docker images, specify your own Docker socket location, choose some flags, test and your upgrade scenarios without needing to consult the docs.

## Prerequisites

The only prerequisite for `bovine` is the [Docker](https://docs.docker.com/get-docker/) daemon.
Customize your Docker installation to your liking since `bovine` does not require the Docker CLI and can use a custom socket path.

## Installation

`bovine` is designed to work on any tier one Rust platform that can interface with Docker.
You can install and [upgrade](./docs/EXTRA.md) the application by using `cargo`.

```sh
cargo install bovine
```

You can install `cargo` via [rustup](https://rustup.rs/) (recommended) or your preferred package manager.

If you do not want to install `cargo`, you can download a binary from the [releases page](https://github.com/nickgerace/bovine/releases).
The following convenience script can be used on macOS and Linux `amd64` systems (requires `wget`, `jq`, and `curl` to be installed):

```sh
(
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    if [ "$OS" = "linux" ]; then OS=linux-gnu; fi
    LATEST=$(curl -s https://api.github.com/repos/nickgerace/bovine/releases/latest | jq -r ".tag_name")
    wget -O bovine https://github.com/nickgerace/bovine/releases/download/$LATEST/bovine-$OS-amd64
    chmod +x bovine
    sudo mv bovine /usr/local/bin/bovine
)
```

## Usage

By default, `bovine run` will create and run a Rancher container with common settings.
See all options with the following command:

```
% bovine run --help
```

> For more information, consult the official [Rancher single node documentation](https://rancher.com/docs/rancher/v2.5/en/installation/other-installation-methods/single-node-docker/).

What about saving your settings for future use?
You can do it with valid JSON.
Let's save it to a file.

```
% bovine run --dry-run > dry.json
```

You can also obtain the config and status for a container, whether it is running or not.
Since this information is bundled into JSON, let's save it to another file.

```
% bovine get ead7ff0c711a > get.json
```

We've probably built a lot of Rancher containers while testing out these commands.
Let's start over from the beginning.

```
% bovine stop --all --delete
Stopped Rancher container: ead7ff0c711a
Deleted Rancher container: ead7ff0c711a
Deleted volumes for container: ead7ff0c711a
Container not modified (may have already been stopped): d39cca6514d8
Deleting Rancher container: d39cca6514d8
Deleted volumes for container: d39cca6514d8
```

There's a new version of Rancher out!
Let's upgrade our `stable` Rancher instance to `latest`.

```
% bovine upgrade bc3ad1bf4fd7 latest
Stopped Rancher container: bc3ad1bf4fd7
Created temporary container for volume backup: b6f3adef1c23
Image found locally: [rancher/rancher:latest]
Rancher container is running: 9cf5f2ead13d
Upgrade from [rancher/rancher:stable] to [rancher/rancher:latest] complete
```

Forgot the name of your one and only `bovine` container?
No problem.

```
% bovine upgrade $(bovine list --short) latest
```

When a new version of Rancher comes out using the `latest` [tag](https://hub.docker.com/r/rancher/rancher/tags) (the default for `bovine run`), you may need to force pull the image.

```
% bovine run --force-pull
```

When using localhost tunneling (e.g [ngrok](https://ngrok.com/)), you may need to set `--no-cacerts` for provisioning to function properly.

```
% bovine run -n
```

If you are working with Rancher >=v2.6, you may need to find the bootstrap password in order to access the dashboard.

```
% bovine logs -p
```

You can also set the bootstrap password upon startup.

```
% bovine run -b <password>
```

## Troubleshooting

If we need to examine a live cluster, we can follow its container logs.

```
% bovine logs 8fccc0c04184 --follow
```

We can also dump the logs into a file.

```
% bovine logs 8fccc0c04184 > bovine.log
```

If you have found a bug that's likely to be unrelated to Rancher, you can pin down your version information to dive deeper.
Let's print that information out, just to get the hang of it.

```
% bovine version
{
  "bovine": {
    "version": "0.1.2",
    "os/arch": "linux/x86_64"
  },
  "docker": {
    "version": "20.10.8",
    "os/arch": "linux/amd64",
    "api_version": "1.41",
    "linux_kernel_version": "5.11.0-27-generic",
    "git_commit": "75249d8"
  }
}

```

If you are using a custom socket path, `bovine` will confirm that the information was gathered from there.
Let's try it on a Linux host.

```
% bovine --docker-socket-path /foo/bar/docker.sock version
{
  "bovine": {
    "version": "0.1.2",
    "os/arch": "linux/x86_64"
  },
  "docker": {
    "version": "20.10.8",
    "os/arch": "linux/amd64",
    "api_version": "1.41",
    "linux_kernel_version": "5.11.0-27-generic",
    "git_commit": "75249d8"
    "socket_path": "/foo/bar/docker.sock"
  }
}
```

Maybe Docker is the issue in your troubleshooting session?
`bovine` will print some version information anyway, just in case.

```
% bovine version
{
  "bovine": {
    "version": "0.1.2",
    "os/arch": "linux/x86_64"
  },
  "docker": {
    "error": "could not connect to docker (check if docker is running)"
  }
}
```

### Windows

If you are having issues with the native Windows binary, the following tips may help:

- [PowerShell](https://github.com/PowerShell/PowerShell) 7.1+ might need to be installed and used when executing the `bovine` binary.
- Instead of accessing `localhost`, users may have to navigate to `host.docker.internal` in their browser of choice.
- Rancher does not support native Windows images for its local cluster at this time, so Docker must be configured to deploy Linux containers.

### Other

You may notice that `bovine` runs Rancher containers in privileged mode.
This is required as of Rancher v2.5 (and is not a `bovine` requirement).
More information can be found in the [official docs](https://rancher.com/docs/rancher/v2.5/en/installation/other-installation-methods/single-node-docker/#privileged-access-for-rancher-v2-5).

## Why should I use this instead of my current workflow?

Let's talk freely here.
Isn't this just a glorified version of Bash scripts with `docker` CLI commands?
`bovine` does ultimately leverage the Docker daemon as its "engine", but there's more to its design than that.
Some notes that may provide context:

- `bovine` may use other container runtimes in the future.
  - In this scenario, users would have the option to choose between multiple runtimes (depending on the host OS).
- Multi-platform support is essential, but especially so for an application focused on being a "first step" into trying out Rancher and/or Kubernetes.
- Error handling, maintainability, UX, and "refactorability" are central to its design.
- Even if the above points were non-existent, `bovine` tries to make the Kubernetes and/or Rancher experience easier for newcomers.
  - Sometimes, a small abstraction makes the difference between a user trying out and hesitantly skipping underlying software.
- `bovine` provides one-button automation, such as stopping, deleting, and removing volumes for containers without affecting other containers and without checking IDs.

## Disclaimer

`bovine` is not an official [SUSE](https://suse.com) or [Rancher Labs](https://rancher.com) product at this time.
While it is intended for "real world use" among other purposes described throughout this `README`, it is independently maintained.
