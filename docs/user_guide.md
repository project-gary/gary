# User Guide

### Notice:
    This is a best effort set of documentation, if it does not work for you, please ether open an issue, wait for an update, join our discord or submit a PR!

## Installing on your first Node

    Node == machine/vm/whatever.

Currently there are no prebuilt binaries. Once we reach pre-alpha these will be maintained. To install you will need rust/cargo. The project can easily be built by running `cargo build` a docker build environment is on the road map but not currently supported.

`cargo build` can also be used to build the `garyctl` client if you would like to use that to interact with your running cluster.

Once you have a `gary` binary copy this to the machines you want to be in your cluster, the [default ports](../core/defaults.rs) will be used if others are not specified in your `~/.gary/config` file. 

To start you gary client you can just run the binary in your favorite shell, if you would like to run it as a daemon run it with `-d`

To start your second node pass the --target or add targets to your config file of at least one of the currently running nodes. The node will connect and you will now have a running cluster.

now you can run `garyctl get nodes --target fqdn-of-any-node` you can pass multiple targets and garyctl will try them sequentially in case of a failure and return a list of all nodes.

Next follow the [deployment guide](deployment_guide.md) to deploy an app to your cluster!