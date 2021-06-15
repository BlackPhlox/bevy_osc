use bevy::{
    core::{CorePlugin, Time, Timer},
    input::Input,
    prelude::{
        info, AppBuilder, Commands, EventReader, EventWriter, IntoSystem, KeyCode,
        ParallelSystemDescriptorCoercion, Plugin, Res, ResMut,
    },
    DefaultPlugins,
};
use nannou_osc as osc;
use osc::{Connected, Receiver, Sender};

struct OSCReceiver {
    receiver: Receiver,
}

struct OSCSender {
    sender: Sender<Connected>,
}

#[derive(Clone)]
pub struct OSCLog {
    pub received_packets: Vec<(std::net::SocketAddr, osc::Packet)>,
}

pub struct OSCEvent {
    pub addr: std::net::SocketAddr,
    pub packet: osc::Packet,
}

pub struct OSCSettings {
    pub max_log_packets: usize,
    pub recv_addr: Option<&'static str>,
    pub send_addr: Option<&'static str>,
    pub dbg: bool,
    pub log: bool,
}

impl Default for OSCSettings {
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

pub struct OSC;
impl Plugin for OSC {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<OSCSettings>()
            .add_startup_system(osc_setup.system())
            .add_event::<OSCEvent>()
            .add_system(osc_listener_update.system());
    }
}

fn osc_setup(mut commands: Commands, settings: Res<OSCSettings>) {
    if settings.recv_addr.is_some() {
        commands.insert_resource(OSCReceiver {
            receiver: Receiver::bind_to(settings.recv_addr.unwrap()).unwrap(), /*osc::receiver(34254).unwrap(),*/
        });
        if settings.dbg {
            println!("OSC Listening on {}", settings.recv_addr.unwrap());
        }
    }

    if settings.send_addr.is_some() {
        commands.insert_resource(OSCSender {
            sender: osc::sender()
                .unwrap()
                .connect(settings.send_addr.unwrap())
                .unwrap(),
        });
        if settings.dbg {
            println!("OSC Sending on {}", settings.send_addr.unwrap());
        }
    }

    commands.insert_resource(OSCLog {
        received_packets: Vec::with_capacity(settings.max_log_packets),
    });
}

fn osc_listener_update(
    rec: Res<OSCReceiver>,
    settings: Res<OSCSettings>,
    mut log: ResMut<OSCLog>,
    mut osc_events: EventWriter<OSCEvent>,
) {
    //if rec.receiver.iter().count() <= 0 {return}

    for (packet, addr) in rec.receiver.recv().iter() {
        if log.received_packets.len() > settings.max_log_packets {
            log.received_packets.remove(0);
        }

        let address = addr.clone();

        //println!("{:?}", packet);
        log.received_packets.push((address, packet.clone()));

        if settings.log {
            osc_log_update(log.clone())
        }

        osc_events.send(OSCEvent {
            addr: address,
            packet: packet.clone(),
        });
    }
}

fn osc_log_update(log: OSCLog) {
    println!("Log");
    for (_add, p) in log.received_packets.iter() {
        println!("{:?}", p);
    }
}
