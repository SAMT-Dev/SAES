use serde::Deserialize;
use socketioxide::extract::SocketRef;
use tracing::{info, warn};

use crate::{ auth::validate_jwt, config::loader::get_config, logging::db_log};

#[derive(Debug, Deserialize)]
pub struct InitialData {
    auth_token: String,
}

pub async fn on_connect(socket: SocketRef, data: InitialData) {
    let jwt = validate_jwt(data.auth_token.clone()).await;
    if jwt.is_some() {
        let jwt = jwt.unwrap();
        info!(
            "Socket {:?} connected: {:?} {:?}",
            socket.id,
            socket.ns(),
            data,
        );

        info!(
            "Socket {} authenticated: {} / {}",
            socket.id, jwt.username, jwt.id,
        );
        db_log(jwt.username.clone(), None, None, None, "LOGIN", None).await;
        let mama = get_config().await;
        if jwt.is_sys_admin {
            socket.join("sysadmin");
        }
        // io.to("socketppl")
        //   .emit("socketppl-update", io.sockets().unwrap().len())
        //   .expect("SocketPPL - Update on connect kiküldése sikertelen");
        socket.join("ucp");
        socket
            .emit("maintenance", &mama.global.maintenance)
            .unwrap();
        socket
            .emit("announcement", &mama.global.announcement)
            .unwrap();
        socket.emit("doneload", "").unwrap();
        //socket.on(
        //    "JoinEvent",
        //   move |s: SocketRef, Data(data): Data<EventData>| {
        //       if data.event_name == "socketppl" {
        //           s.join(data.event_name).unwrap();
        //           s.emit("socketppl-update", iod).expect("Fasz van");
        //      }
        //  },
        // )//;
        // socket.on(
        //     "LeaveEvent",
        //     move |s: SocketRef, Data(data): Data<EventData>| {
        //        if data.event_name == "socketppl" {
        //            s.leave(data.event_name).unwrap();
        //       }
        //    },
        // );
        socket.on_disconnect(move |s: SocketRef| {
            info!("Socket {} disconnected {} / {}", s.id, jwt.username, jwt.id);
            //io.to("socketppl")
            //   .emit("socketppl-update", iod - 1)
            //   .expect("SocketPPL - Update on disconnect kiküldése sikertelen");
        });
    } else {
        warn!("Socket {} érvénytelen lekérés", socket.id);
        return socket.disconnect().unwrap();
    }
}
