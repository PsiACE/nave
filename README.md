# Nave

[![Crates.io](https://img.shields.io/crates/v/nave.svg)](https://crates.io/crates/nave)
[![Docs](https://docs.rs/nave/badge.svg)](https://docs.rs/nave)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/nave.svg)](https://crates.io/crates/nave)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/PsiACE/nave/Check%20Code?label=workflow)](https://github.com/PsiACE/nave/actions)

> Thinking about the construction of distributed systems starting from the *consistent hash* algorithm.

**Nave**'s vision is to become the basis for building a robust and highly available distributed system. The possible final product is a collection of important algorithms or a framework in a certain sense.

You can already use it in projects, of course, currently only a port of *[rust-hash-ring](https://github.com/mattnenterprise/rust-hash-ring)*.

## Features

- Port *[rust-hash-ring](https://github.com/mattnenterprise/rust-hash-ring)*, use xxhash to replace md5.

## Usage

- **HashRing**

  ```rust
  extern crate nave;

  use nave::HashRing;

  /// Custom Node Info. 
  #[derive(Clone, Debug)]
  struct NodeInfo {
      pub host: &'static str,
      pub port: u16,
  }

  /// Impl ToString to format NodeInfo
  impl ToString for NodeInfo {
      fn to_string(&self) -> String {
          format!("{}:{}", self.host, self.port)
      }
  }

  fn main() {
      let mut nodes: Vec<NodeInfo> = Vec::new();
      nodes.push(NodeInfo { host: "localhost", port: 15324 });
      nodes.push(NodeInfo { host: "localhost", port: 15325 });
      nodes.push(NodeInfo { host: "localhost", port: 15326 });
      nodes.push(NodeInfo { host: "localhost", port: 15327 });
      nodes.push(NodeInfo { host: "localhost", port: 15328 });
      nodes.push(NodeInfo { host: "localhost", port: 15329 });

      let mut hash_ring: HashRing<NodeInfo> = HashRing::new(nodes, 10);

      println!("{}", hash_ring.get_node(("hello").to_string()).unwrap().to_string());

      println!("{}", hash_ring.get_node(("dude").to_string()).unwrap().to_string());

      println!("{}", hash_ring.get_node(("martian").to_string()).unwrap().to_string());

      println!("{}", hash_ring.get_node(("tardis").to_string()).unwrap().to_string());

      hash_ring.remove_node(&NodeInfo { host: "localhost", port: 15329 });

      println!("{}", hash_ring.get_node(("hello").to_string()).unwrap().to_string());

      hash_ring.add_node(&NodeInfo { host: "localhost", port: 15329 });

      println!("{}", hash_ring.get_node(("hello").to_string()).unwrap().to_string());
  }
  ```

## Contact

Chojan Shang - [@PsiACE](https://github.com/psiace) - <psiace@outlook.com>

Project Link: [https://github.com/psiace/nave](https://github.com/psiace/nave)

## Sponsor

If my work can make you feel happy, you can consider buying me a cup of coffee:

- Paypal: [https://paypal.me/psiace](https://paypal.me/psiace)
- Afdian: [https://afdian.net/@psiace](https://afdian.net/@psiace) [爱发电 - 国内]

## License

Licensed under either of

  - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
  - MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
