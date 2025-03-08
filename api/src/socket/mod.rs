use serde::Deserialize;
use socketioxide::extract::SocketRef;
use tracing::{info, warn};

use crate::{
    auth::get_discord_envs,
    config::loader::get_config,
    logging::db_log,
    utils::{
        api::get_api_envs,
        middle::{DiscordUser, Driver, GetUserRes},
    },
    WEB_CLIENT,
};

pub mod app;

#[derive(Debug, Deserialize)]
pub struct InitialData {
    auth_token: String,
}

pub async fn on_connect(socket: SocketRef, data: InitialData) {
    info!(
        "Socket {:?} connected: {:?} {:?}",
        socket.id,
        socket.ns(),
        data,
    );
    let ds = get_discord_envs().await;
    let envs = get_api_envs().await;
    let dcuserget = WEB_CLIENT
        .get(format!("{}/users/@me", ds.api_endpoint))
        .header("Authorization", format!("Bearer {}", data.auth_token))
        .send()
        .await
        .expect("Lekérés sikertelen");
    if dcuserget.status().as_u16() == 200 {
        let handled_user = dcuserget.text().await.expect("Átalakítás sikertelen");
        let parsed_user = serde_json::from_str(&handled_user);
        if parsed_user.is_ok() {
            let real_user: DiscordUser = parsed_user.unwrap();
            let getuser: String = WEB_CLIENT
                .get(format!("{}/authenticate", envs.fms,))
                .query(&[("dcid", real_user.id.clone())])
                .header("authkey", envs.fms_key)
                .send()
                .await
                .expect("Lekérés sikertelen")
                .text()
                .await
                .expect("Átalakítás sikertelen");
            let parsed_tag = serde_json::from_str(&getuser);
            if parsed_tag.is_ok() {
                let real_tag: GetUserRes = parsed_tag.unwrap();
                let tag = Driver {
                    discordid: real_user.id,
                    name: real_tag.username,
                    driverid: real_tag.id,
                    admin: real_tag.issysadmin,
                    perms: real_tag.permissions,
                    faction: None,
                    taxi: real_tag
                        .factionrecords
                        .iter()
                        .find(|fact| fact.factionid == 1)
                        .cloned(),
                    tow: real_tag
                        .factionrecords
                        .iter()
                        .find(|fact| fact.factionid == 3)
                        .cloned(),
                    apms: real_tag
                        .factionrecords
                        .iter()
                        .find(|fact| fact.factionid == 2)
                        .cloned(),
                };
                info!(
                    "Socket {} authenticated: {} / {} / {}",
                    socket.id, tag.name, tag.driverid, tag.discordid,
                );
                db_log(tag.name.clone(), None, None, None, "LOGIN", None).await;
                let mama = get_config().await;
                if tag.admin {
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
                    info!(
                        "Socket {} disconnected {} / {} / {}",
                        s.id, tag.name, tag.driverid, tag.discordid
                    );
                    //io.to("socketppl")
                    //   .emit("socketppl-update", iod - 1)
                    //   .expect("SocketPPL - Update on disconnect kiküldése sikertelen");
                });
            } else {
                warn!("Socket {} nincs joga", socket.id);
                return socket.disconnect().unwrap();
            }
        } else {
            warn!("Socket {} érvénytelen lekérés", socket.id);
            return socket.disconnect().unwrap();
        }
    } else {
        warn!("Socket {} érvénytelen lekérés", socket.id);
        return socket.disconnect().unwrap();
    }
}
