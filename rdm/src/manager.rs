/*use std::io::Write;

use dbus::{BusType, Connection, ConnectionItem, Error, Message, MessageItem, NameFlag};
use dbus::obj::{Argument, Interface, Method, ObjectPath, Property};

use constants::*;

pub struct Manager {
    running: bool,
}

impl Manager {
    pub fn new() -> Manager {
        Manager { running: false }
    }

    pub fn start(self) {
        let conn = match Connection::get_private(BusType::System) {
            Ok(c) => c,
            Err(e) => panic!("Manager: Failed to get DBUS connection: {:?}", e),
        };
        info!("Manager: Opened {:?}", conn);

        conn.register_name(DBUS_SERVICE_NAME, NameFlag::ReplaceExisting as u32).unwrap();
        info!("Manager: Registered service name {}", DBUS_SERVICE_NAME);

        let root_iface = Interface::new(vec![Method::new("Hello",
                                                         vec![],
                                                         vec![Argument::new("reply", "s")],
                                                         Box::new(|msg| {
                                                             Ok(vec!(MessageItem::Str(format!("Hello {}!", msg.sender().unwrap()))))
                                                         }))],
                                        vec![],
                                        vec![]);
        let mut root_path = ObjectPath::new(&conn, "/", true);
        root_path.insert_interface(DBUS_ROOT_PATH, root_iface);
        root_path.set_registered(true).unwrap();
        info!("Manager: Registered interface!");

        info!("Manager: Starting main loop!");
        for n in conn.iter(1) {
            if let ConnectionItem::MethodCall(mut m) = n {
                if root_path.handle_message(&mut m).is_none() {
                    conn.send(Message::new_error(&m,
                                                 "org.freedesktop.DBus.Error.Failed",
                                                 "Object path not found")
                            .unwrap())
                        .unwrap();
                    info!("Path not found");
                } else {
                    info!("Handled method call!");
                }
            };
        }
        info!("Manager: Quit main loop. Exiting..");
    }
}*/