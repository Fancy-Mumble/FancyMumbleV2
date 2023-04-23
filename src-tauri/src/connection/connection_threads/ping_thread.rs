use crate::{
    connection::{Connection, PingThread},
    utils::messages::{message_builder, mumble},
};
use std::time::{SystemTime, Duration};
use tokio::time;

const PING_INTERVAL: Duration = Duration::from_millis(5000);

impl PingThread for Connection {
    fn spawn_ping_thread(&mut self) {
        let tx_a = self.tx_out.clone();
        let running_clone = self.running.clone();

        // timer thread
        self.threads.ping_thread = Some(tokio::spawn(async move {
            let mut interval = time::interval(PING_INTERVAL);
            while *running_clone.read().unwrap() {
                let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
                if now.is_err() {
                    println!("Unable to send Ping!");
                    continue;
                }

                //todo: Add actual ping statistics
                let ping = mumble::proto::Ping {
                    timestamp: Some(now.unwrap().as_secs()),
                    good: Some(0),
                    late: Some(0),
                    lost: Some(999),
                    resync: Some(0),
                    tcp_packets: Some(1),
                    tcp_ping_avg: Some(1.2f32),
                    tcp_ping_var: Some(2.0f32),
                    udp_packets: Some(1),
                    udp_ping_avg: Some(2.3f32),
                    udp_ping_var: Some(5.6f32),
                };

                interval.tick().await;
                let buffer = message_builder(ping);
                _ = tx_a.send(buffer);
            }
        }));
    }
}
