use bevy::{core::{Time, Timer}, prelude::{AppBuilder, Commands, EventReader, EventWriter, IntoSystem, Plugin, Res, ResMut, info}};
use nannou_osc as osc;
use osc::Receiver;

struct OSCReceiver {
    receiver: Receiver,
}

#[derive(Clone)]
pub struct OSCLog {
    pub received_packets: Vec<(std::net::SocketAddr, osc::Packet)>,
}

pub struct OSCSettings {
    pub max_packets: usize,
    pub addr: &'static str,
    pub dbg: bool,
}

impl Default for OSCSettings {
    fn default() -> Self {
        Self {
            max_packets: 5,
            addr: "127.0.0.1:34254",
            dbg: true,
        }
    }
}

pub struct OSC;
impl Plugin for OSC {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<OSCSettings>()
            .add_startup_system(osc_setup.system())
            .add_system(osc_listener_update.system())

            .add_event::<MyEvent>()
            .add_system(event_listener_system.system());
    }
}

fn osc_setup(mut commands: Commands, settings: Res<OSCSettings>) {
    commands.insert_resource(OSCReceiver {
        receiver: Receiver::bind_to(settings.addr).unwrap(), /*osc::receiver(34254).unwrap(),*/
    });

    commands.insert_resource(OSCLog {
        received_packets: Vec::with_capacity(settings.max_packets),
    });

    if settings.dbg {
        println!("OSC Listening on {}", settings.addr);
    }
}

fn osc_listener_update(rec: Res<OSCReceiver>, settings: Res<OSCSettings>, mut log: ResMut<OSCLog>, mut my_events: EventWriter<MyEvent>,) {
    for (packet, addr) in rec.receiver.iter() {
        
        //println!("{:?}",packet);
        if log.received_packets.len() > settings.max_packets {
            log.received_packets.remove(0);
        }
        
        //println!("{:?}", packet);
        log.received_packets.push((addr, packet));
        
        osc_log_update(log.clone());
        
        my_events.send(MyEvent {
            message: "MyEvent just happened!".to_string(),
        });
    }
}

fn osc_log_update(log: OSCLog) {
    println!("Log");
    for (_add, p) in log.received_packets.iter() {
        println!("{:?}", p);
    }
}

pub struct MyEvent {
    pub message: String,
}

// prints events as they come in
fn event_listener_system(mut events: EventReader<MyEvent>, ) {
    for my_event in events.iter() {
        println!("Test");
        info!("{}", my_event.message);
    }
}
