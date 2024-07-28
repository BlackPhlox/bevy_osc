use bevy::{app::{Startup, Update}, prelude::{App, Commands, Deref, Event, EventWriter, Plugin, Res, ResMut, Resource}};
use nannou_osc as osc;
use osc::{Connected, Receiver, Sender};

#[derive(Resource)]
pub struct OscReceiver {
    receiver: Receiver,
}

#[derive(Resource, Deref)]
#[allow(dead_code)]
pub struct OscSender {
    pub sender: Sender<Connected>,
}

#[derive(Resource, Clone)]
pub struct OscLog {
    pub received_packets: Vec<(std::net::SocketAddr, osc::Packet)>,
}

#[derive(Event)]
pub struct OscEvent {
    pub addr: std::net::SocketAddr,
    pub packet: osc::Packet,
}

#[derive(Resource)]
pub struct OscSettings {
    pub max_log_packets: usize,
    pub recv_addr: Option<&'static str>,
    pub send_addr: Option<&'static str>,
    pub dbg: bool,
    pub log: bool,
}

impl Default for OscSettings {
    fn default() -> Self {
        Self {
            recv_addr: Some("127.0.0.1:34254"),
            send_addr: None,
            dbg: true,
            log: true,
            max_log_packets: 5,
        }
    }
}

pub struct Osc;
impl Plugin for Osc {
    fn build(&self, app: &mut App) {
        app.init_resource::<OscSettings>()
            .add_systems(Startup, osc_setup)
            .add_event::<OscEvent>()
            .add_systems(Update, osc_listener_update);
    }
}

fn osc_setup(mut commands: Commands, settings: Res<OscSettings>) {
    if settings.recv_addr.is_some() {
        commands.insert_resource(OscReceiver {
            receiver: Receiver::bind_to(settings.recv_addr.unwrap()).unwrap(), /*osc::receiver(34254).unwrap(),*/
        });
        if settings.dbg {
            println!("OSC Listening on {}", settings.recv_addr.unwrap());
        }
    }

    if settings.send_addr.is_some() {
        commands.insert_resource(OscSender {
            sender: osc::sender()
                .unwrap()
                .connect(settings.send_addr.unwrap())
                .unwrap(),
        });
        if settings.dbg {
            println!("OSC Sending on {}", settings.send_addr.unwrap());
        }
    }

    commands.insert_resource(OscLog {
        received_packets: Vec::with_capacity(settings.max_log_packets),
    });
}

fn osc_listener_update(
    rec: Res<OscReceiver>,
    settings: Res<OscSettings>,
    mut log: ResMut<OscLog>,
    mut osc_events: EventWriter<OscEvent>,
) {
    for (packet, addr) in rec.receiver.try_iter() {
        if log.received_packets.len() > settings.max_log_packets {
            log.received_packets.remove(0);
        }

        let address = addr;

        log.received_packets.push((address, packet.clone()));

        if settings.log {
            osc_log_update(log.clone())
        }

        osc_events.send(OscEvent {
            addr: address,
            packet: packet.clone(),
        });
    }
}

fn osc_log_update(log: OscLog) {
    println!("Log");
    for (_add, p) in log.received_packets.iter() {
        println!("{:?}", p);
    }
}
