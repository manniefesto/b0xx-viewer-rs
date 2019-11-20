use crate::b0xx_state::*;
use crate::error::ViewerError;

#[cfg_attr(feature = "fake_serial", allow(dead_code))]
#[derive(Debug)]
pub enum B0xxMessage {
    State(B0xxState),
    Error(ViewerError),
    Reconnect,
    Quit,
}

#[inline]
pub fn reconnect() -> crossbeam_channel::Receiver<B0xxMessage> {
    loop {
        if let Ok(new_rx) = start_serial_probe() {
            return new_rx;
        }
    }
}

#[cfg(not(feature = "fake_serial"))]
pub fn start_serial_probe() -> Result<crossbeam_channel::Receiver<B0xxMessage>, ViewerError> {
    use std::io::Read;

    let b0xx_port = serialport::available_ports()?
        .into_iter()
        .find(|port| {
            if let serialport::SerialPortType::UsbPort(portinfo) = &port.port_type {
                if portinfo.vid == 9025 && portinfo.pid == 32822 {
                    return true;
                }

                if let Some(product) = &portinfo.product {
                    if product == "Arduino_Leonardo" {
                        return true;
                    }
                }
            }

            false
        })
        .ok_or_else(|| ViewerError::B0xxNotFound)?;

    let port_settings = serialport::SerialPortSettings {
        baud_rate: 115_200,
        data_bits: serialport::DataBits::Eight,
        flow_control: serialport::FlowControl::Hardware,
        parity: serialport::Parity::None,
        stop_bits: serialport::StopBits::One,
        timeout: std::time::Duration::from_millis(500),
    };

    let wait = std::time::Duration::from_micros(8200);

    let (tx, rx) = crossbeam_channel::unbounded();
    std::thread::Builder::new()
        .name("b0xx_viewer_serial".into())
        .spawn(move || {
            let mut buf = Vec::with_capacity(18);

            let mut port =
                match serialport::open_with_settings(&b0xx_port.port_name, &port_settings) {
                    Ok(port) => port,
                    Err(e) => return tx.send(B0xxMessage::Error(e.into())),
                };

            match port.write_request_to_send(true) {
                Err(e) => return tx.send(B0xxMessage::Error(e.into())),
                _ => {}
            }

            let loop_tx = tx.clone();
            let mut schedule_to_send = false;

            if let Err(e) = port.bytes().try_for_each(
                move |b: Result<u8, std::io::Error>| -> Result<(), ViewerError> {
                    use std::convert::TryInto as _;

                    let report: B0xxReport = b?.into();
                    match report {
                        B0xxReport::End => {
                            if let Ok(state) = buf.as_slice().try_into() {
                                let _ = loop_tx.try_send(B0xxMessage::State(state));
                                buf.clear();
                            } else {
                                schedule_to_send = true;
                            }
                        }
                        B0xxReport::Invalid => {}
                        _ => {
                            if buf.len() < buf.capacity() {
                                buf.push(report);
                            } else if schedule_to_send {
                                if let Ok(state) = buf.as_slice().try_into() {
                                    let _ = loop_tx.try_send(B0xxMessage::State(state));
                                }

                                buf.clear();
                                buf.push(report);
                                schedule_to_send = false;
                            }
                        }
                    }

                    std::thread::sleep(wait);

                    Ok(())
                },
            ) {
                match &e {
                    ViewerError::IoError(io_error) => match io_error.kind() {
                        std::io::ErrorKind::TimedOut | std::io::ErrorKind::BrokenPipe => {
                            return tx.send(B0xxMessage::Reconnect);
                        }
                        _ => {
                            error!("{:?}", ViewerError::from(e));
                            return tx.send(B0xxMessage::Quit);
                        }
                    },
                    _ => {
                        error!("{:?}", ViewerError::from(e));
                        return tx.send(B0xxMessage::Quit);
                    }
                }
            }

            Ok(())
        })?;

    Ok(rx)
}

#[cfg(feature = "fake_serial")]
pub fn start_serial_probe() -> Result<crossbeam_channel::Receiver<B0xxMessage>, ViewerError> {
    let (tx, rx) = crossbeam_channel::unbounded();
    let wait = std::time::Duration::from_micros(8200);
    std::thread::spawn(move || loop {
        let _ = tx.send(B0xxMessage::State(B0xxState::random()));
        std::thread::sleep(wait);
    });

    Ok(rx)
}
