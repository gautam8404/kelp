/// UCI trait, implements basic functions for UCI protocol, all handle_* functions are passed with array of args excluding the keyword
pub trait UCI {
    fn handle_position(&mut self, arg: &[&str]);

    fn handle_uci_newgame(&mut self);
    fn handle_go(&mut self, arg: &[&str]);

    fn handle_uci(&self, arg: &[&str]);

    fn handle_quit(&self);

    fn handle_stop(&self);

    fn handle_ready(&self);

    // To handle commands that are not implemented by default in trait
    fn handle_unknown(&self, command: &str, arg: &[&str]);

    fn print_board(&self);
    fn is_keyword(&self, arg: &str) -> bool {
        matches!(
            arg,
            "position" | "go" | "uci" | "quit" | "stop" | "ponderhit" | "debug" | "isready"
        )
    }

    fn is_position(&self, arg: &str) -> bool {
        matches!(arg, "startpos" | "fen")
    }

    fn is_go(&self, arg: &str) -> bool {
        matches!(
            arg,
            "searchmoves"
                | "ponder"
                | "wtime"
                | "btime"
                | "winc"
                | "binc"
                | "movestogo"
                | "depth"
                | "nodes"
                | "mate"
                | "movetime"
                | "infinite"
        )
    }

    fn receive(&mut self, arg: &str) {
        self.log_stdio(&format!("Received: {}", arg));
        let mut args = arg.split_whitespace().collect::<Vec<&str>>();
        if args.is_empty() {
            return;
        }
        let command = args.remove(0);
        match command {
            "position" => self.handle_position(&args),
            "go" => self.handle_go(&args),
            "uci" => self.handle_uci(&args),
            "quit" => self.handle_quit(),
            "stop" => self.handle_stop(),
            "isready" => self.handle_ready(),
            "ucinewgame" => self.handle_uci_newgame(),
            "d" => self.print_board(),
            _ => self.handle_unknown(command, &args),
        }
    }

    fn send(&self, arg: &str) {
        self.log_stdio(&format!("Sent: {}", arg));
        println!("{}", arg);
    }

    fn send_ready_ok(&self) {
        self.send("readyok");
    }

    fn send_bestmove(&self, bestmove: &str) {
        self.send(format!("bestmove {}", bestmove).as_str());
    }

    fn send_info(&self, info: &str) {
        self.send(format!("info {}", info).as_str());
    }

    // a very basic uci loop, can be overriden
    fn uci_loop(&mut self) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.receive(input.trim());
        }
    }

    fn log_stdio(&self, _arg: &str) {} // Optional function to log stdio
}
