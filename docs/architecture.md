# Gary Architecture

Gary is designed from the ground up to be designed as a container orchestrator that uses freedom as a key principle.

- no control plane. your cluster will operate (as long as it has enough nodes to do all the work!) no mater what nodes fail. no compromises. This will be initially accomplished using Promise Theory.
- speed and efficiency. Gary is designed from the ground up to have the minimum overhead. You pay for your hardware, your orchestrator should not waste them especially when it runs on every machine.
- Adherence to Unix philosophies, in particular:
    * Make each program do one thing, and do it well (aka, we will NOT make a service mesh included, that is a separate issue.)
    * Make sure you write things to work together with other applications
    * Ensure everything is easy to test.
- Dependency injection/modular design as first class citizens. Aka, Even our node to node communication (initial implementation to be done using nanomsg) should have the nanomsg parts hidden behind a well defined/implementable interface enabling a future developer that wants to create node to node communication using gRPC they would only need to implement the interface and build using their module.
- Rethink everything. While doing it the same way as kubernetes or nomad might be fine and acceptable. Our goal is to rethink even their core concepts.

## Why Rust
Container orchestration tools have been dominated by GoLang. Why deviate? well, there are a lot of reasons. You can read a lot of blog posts about this but here is a very short list of some of them.
* Rust is more expressive
* Rust has better performance most of the time.
* Rust provides safety and security principles that are required for a container orchestration tool

#### Why not C++ or C then?
Being an open source project we wanted a language that was easier to get into. C++ and C does not offer the memory security and safety that we desired. I love C++, probably more then most.

## The Design
If the above did not scare you off, here is the high level design. More technical breakouts will be described at a later date.

Each node will be it's own free state, But it is a trusted free state. Meaning when it makes a promise, you can trust that it will do everything in it's power to keep that promise.

So, Each node can and will promise other nodes that it will do something, say for example run a container.

Deployments will be self contained. So a deployment of 20 containers running on 15 nodes is only known to those 15 nodes. All the other nodes will only know the nodes responsible.

In the above, if one of the 15 nodes dies. the cluster of 15 (now 14) will handle asking for some new node to volunteer to become part of the cluster running that deployment.

