/// UCI trait, implements basic functions for UCI protocol, all handle_* functions are passed with array of args excluding the keyword
pub trait UCI {
    fn handle_position(&mut self, arg: &[&str]);
    fn handle_go(&mut self, arg: &[&str]);

    fn handle_uci(&self, arg: &[&str]);

    fn handle_quit(&self, arg: &[&str]);

    fn handle_stop(&self, arg: &[&str]);

    fn handle_ready(&self, arg: &[&str]);

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
            "quit" => self.handle_quit(&args),
            "stop" => self.handle_stop(&args),
            "isready" => self.handle_ready(&args),
            "ucinewgame" => self.handle_position(["startpos"].as_ref()),
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

    fn uci_loop(&mut self) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.receive(input.trim());
        }
    }

    fn log_stdio(&self, arg: &str) {} // Optional function to log stdio
}
