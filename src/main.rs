use std::time::Duration;

use dbus::arg::PropMap;
use dbus::channel::{Sender, Channel};
use dbus::{blocking::Connection, message::MatchRule, Message, channel::MatchingReceiver};

use nix::unistd::{Uid, seteuid, geteuid};

use clap::Parser;

static APP_ICON: &str = "utilities-system-monitor";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Owner of the bus
    #[clap(short, long, default_value_t = 1000, value_parser)]
    uid: u32,

    /// Bus address [default: unix:path=/run/user/1000/bus]
    #[clap(short, long, value_parser)]
    address: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();

    let uid: u32 = cli.uid;

    let address = cli.address.unwrap_or_else(|| {
        format!("unix:path=/run/user/{}/bus", uid)
    });

    // TODO: log::debug!("Connecting to system bus...")
    let system_connection = Connection::new_system()?;

    // TODO: log::debug!("Connecting to user bus...")
    let user_connection = connect_address(&address, uid)?;

    let rule = MatchRule::new()
        .with_type(dbus::MessageType::Signal)
        .with_interface("net.nuetzlich.SystemNotifications")
        .with_member("Notify");

    system_connection
        .with_proxy("org.freedesktop.DBus", "/org/freedesktop/DBus", Duration::from_millis(5000))
        .method_call("org.freedesktop.DBus.Monitoring", "BecomeMonitor", (vec!(rule.match_str()), 0u32))?;

    let _id = system_connection.start_receive(rule, Box::new(move |msg, _| {
        redirect(&msg, &user_connection).unwrap();
        true
    }));

    let timeout = Duration::from_millis(1000);

    loop {
        if let Err(e) = system_connection.process(timeout) {
            eprintln!("{}", e);
        }

        // TODO: Pull out gently (i'm so so sorry if you see this)
    }

    Ok(())
}

fn connect_address(address: &str, uid: u32) -> Result<Connection, Box<dyn std::error::Error>> {
    let old_uid = geteuid();
    seteuid(Uid::from_raw(uid)).expect("setting the effective group ID to ${uid}");

    let mut channel = Channel::open_private(address)?;
    channel.register()?;

    seteuid(old_uid).expect("setting the effective group ID to ${old_uid}");

    Ok(channel.into())
}

fn redirect<S>(msg: &Message, sender: &S) -> Result<(), Box<dyn std::error::Error>>
    where
        S: Sender,
{
    let mut arg_iter = msg.iter_init();

    if let Ok(summary) = arg_iter.read::<String>() {

        let body = arg_iter.read::<String>().unwrap_or_default();

        // TODO: log::debug!("{summary} {body}")

        notify(sender, &summary, &body)?
    }

    Ok(())
}

fn notify(sender: &impl Sender, summary: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let actions: Vec<String> = Vec::new();
    let hints = PropMap::new();

    let args = ("system",
                0u32,
                APP_ICON,
                summary,
                body,
                actions,
                hints,
                -1i32);

    // Specification:
    // https://specifications.freedesktop.org/notification-spec/latest/ar01s09.html
    // org.freedesktop.Notifications.Notify
    let msg = Message::call_with_args(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
        "Notify", args);

    let _serial = sender.send(msg).expect("sending message");

    Ok(())
}
