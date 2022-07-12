use crate::b0xx_state::*;
use crate::error::ViewerError;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WhitelistFile {
    arduino: Vec<UsbStringDef>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UsbStringDef {
    pub vid: String,
    pub pid: String,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct UsbDefinition {
    pub vid: u16,
    pub pid: u16,
}

impl std::convert::TryFrom<UsbStringDef> for UsbDefinition {
    type Error = std::num::ParseIntError;

    fn try_from(def: UsbStringDef) -> Result<Self, Self::Error> {
        Ok(Self {
            pid: u16::from_str_radix(def.pid.trim_start_matches("0x"), 16)?,
            vid: u16::from_str_radix(def.vid.trim_start_matches("0x"), 16)?,
        })
    }
}

const ARDUINO_WHITELIST_BYTES: &[u8] = include_bytes!("../assets/arduino_whitelist.toml");
lazy_static! {
    static ref ARDUINO_WHITELIST: Vec<UsbDefinition> = {
        let res: WhitelistFile = toml::from_slice(ARDUINO_WHITELIST_BYTES).unwrap();
        use std::convert::TryFrom as _;
        res.arduino
            .into_iter()
            .map(|s_def| UsbDefinition::try_from(s_def).unwrap())
            .collect()
    };
}

#[cfg_attr(feature = "fake_serial", allow(dead_code))]
#[derive(Debug)]
pub enum B0xxMessage {
    State(B0xxState),
    Error(ViewerError),
    Reconnect,
    Quit,
}

#[inline]
pub fn reconnect(custom_tty: &Option<String>) -> crossbeam_channel::Receiver<B0xxMessage> {
    use backoff::backoff::Backoff as _;
    let mut backoff = backoff::ExponentialBackoff::default();
    loop {
        if let Ok(new_rx) = start_serial_probe(custom_tty) {
            return new_rx;
        }

        if let Some(backoff_duration) = backoff.next_backoff() {
            std::thread::sleep(backoff_duration);
        }
    }
}

#[cfg(not(feature = "fake_serial"))]
pub fn start_serial_probe(
    custom_tty: &Option<String>,
) -> Result<crossbeam_channel::Receiver<B0xxMessage>, ViewerError> {
    let b0xx_port = serialport::available_ports()?
        .into_iter()
        .find(move |port| {
            if let Some(custom_tty) = custom_tty {
                if port.port_name == *custom_tty {
                    return true;
                }
            } else if let serialport::SerialPortType::UsbPort(portinfo) = &port.port_type {
                if std::env::var("RELAX_ARDUINO_DETECT").is_ok() {
                    if ARDUINO_WHITELIST
                        .iter()
                        .any(|def| def.vid == portinfo.vid && def.pid == portinfo.pid)
                    {
                        return true;
                    }
                } else if portinfo.vid == 9025 && portinfo.pid == 32822 {
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

    info!("Found B0XX on port {}", b0xx_port.port_name);

    let (tx, rx) = crossbeam_channel::bounded(1);

    std::thread::Builder::new()
        .name("b0xx_viewer_serial".into())
        .spawn(move || {
            let mut buf = Vec::with_capacity(25);
            let mut state = [B0xxReport::default(); 25];

            let port_builder = serialport::new(&b0xx_port.port_name, 115_200)
                .data_bits(serialport::DataBits::Eight)
                .flow_control(serialport::FlowControl::Hardware)
                .parity(serialport::Parity::None)
                .stop_bits(serialport::StopBits::One)
                .timeout(std::time::Duration::from_millis(500));

            let mut port =
                match port_builder.open() {
                    Ok(port) => port,
                    Err(e) => return tx.send(B0xxMessage::Error(e.into())),
                };


            exhaust_buffer(&mut port, &tx);

            let mut port = std::io::BufReader::with_capacity(25, port);

            use std::io::BufRead as _;
            loop {
                if let Err(e) = port.get_mut().write_request_to_send(true) {
                    return tx.send(B0xxMessage::Error(e.into()));
                }

                let bytes_read: usize = match port.read_until(B0xxReport::End as u8, &mut buf).map_err(Into::into) {
                    Ok(bytes) => bytes,
                    Err(e) => match &e {
                        ViewerError::IoError(io_error) => match io_error.kind() {
                            std::io::ErrorKind::TimedOut | std::io::ErrorKind::BrokenPipe => {
                                return tx.send(B0xxMessage::Reconnect);
                            }
                            _ => {
                                error!("{:?}", e);
                                return tx.send(B0xxMessage::Quit);
                            }
                        },
                        _ => {
                            error!("{:?}", e);
                            return tx.send(B0xxMessage::Quit);
                        }
                    }
                };

                if let Err(e) = port.get_mut().write_request_to_send(false) {
                    return tx.send(B0xxMessage::Error(e.into()));
                }

                trace!("Bytes read: {}", bytes_read);

                port.consume(bytes_read);
                if bytes_read == 25 {
                    for i in 0..25 {
                        state[i] = buf[i].into();
                    }
                } else {
                    exhaust_buffer(port.get_mut(), &tx);
                }

                buf.clear();

                if tx.send(B0xxMessage::State(state.into())).is_err() {
                    info!("Reconnection detected, exiting runloop");
                    return Ok(());
                }
            }
        })?;

    Ok(rx)
}

#[cfg(feature = "fake_serial")]
pub fn start_serial_probe(
    _: &Option<String>,
) -> Result<crossbeam_channel::Receiver<B0xxMessage>, ViewerError> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    if std::env::var("RELAX_ARDUINO_DETECT").is_ok() {
        info!("{:#?}", *ARDUINO_WHITELIST)
    }
    std::thread::spawn(move || loop {
        let _ = tx.send(B0xxMessage::State(B0xxState::random()));
        #[cfg(not(feature = "benchmark"))]
        std::thread::sleep(std::time::Duration::from_micros(16670));
    });

    Ok(rx)
}

#[allow(dead_code)]
#[inline(always)]
fn exhaust_buffer(port: &mut Box<dyn serialport::SerialPort>, tx: &crossbeam_channel::Sender<B0xxMessage>) {
    // Exhaust the initial buffer till we find the end of a report and consume it.
    // This is caused by a UB in Windows' COM port handling causing partial reports
    // sometimes
    trace!("Buffer exhaustion started");
    let mut exhaust_buffer = [0u8; 1];
    use std::io::Read as _;
    loop {
        if let Err(e) = port
            .read_exact(&mut exhaust_buffer)
            .map_err(ViewerError::from)
        {
            error!("{:?}", e);
            let _ = tx.send(B0xxMessage::Quit);
            break;
        }

        if exhaust_buffer[0] == B0xxReport::End as u8 {
            trace!("Buffer exhausted successfully, continuing...");
            break;
        }
    }

    if let Err(e) = port.clear(serialport::ClearBuffer::All) {
        let _ = tx.send(B0xxMessage::Error(e.into()));
    }
}
