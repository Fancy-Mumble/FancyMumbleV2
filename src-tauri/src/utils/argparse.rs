use argparse::{ArgumentParser, Store};

pub fn process_args() -> (String, u16, String) {
    let mut server_host = "magical.rocks".to_string();
    let mut server_port = 64738u16;
    let mut user_name = "Endor".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Connect to a mumble server.");
        ap.refer(&mut server_host)
            .add_option(&["--host"], Store, "hostname");
        ap.refer(&mut server_port)
            .add_option(&["-p", "--port"], Store, "server port");
        ap.refer(&mut user_name)
            .add_option(&["-u", "--user"], Store, "username");
        ap.parse_args_or_exit();
    }

    (server_host, server_port, user_name)
}