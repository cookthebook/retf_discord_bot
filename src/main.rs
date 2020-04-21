use std::{ env, error };

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

#[derive(Debug, Clone)]
struct Handler {
    prefix: String,
    ascii_art: String,
    width: usize,
}

#[derive(Debug, Clone)]
struct SizeError;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        println!("Received message: {:?}", msg.content);

        let resp: String;

        if msg.content == self.prefix {
            resp = self.box_me("I'm the Rust daddy UwU");
        }
        else if msg.content.len() > self.prefix.len() &&
                msg.content[..self.prefix.len()] == self.prefix
        {
            resp = self.box_me(&msg.content[self.prefix.len() + 1..]);
        } else {
            return;
        }

        if let Err(why) = msg.channel_id.say(&ctx.http, resp.as_str()) {
            println!("Error sending message {:?}", why);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

impl Handler {
    const PADDING: usize = 5;

    fn new(prefix: &str, art: &str, text_width: usize)
        -> Result<Handler, Box<dyn error::Error>>
    {
        if text_width <= Handler::PADDING {
            return Err(Box::from(SizeError));
        }

        Ok(
            Handler {
                prefix: String::from(prefix),
                ascii_art: String::from(art),
                width: text_width - Handler::PADDING,
            }
        )
    }


    /*
    +------------+
    | my message |
    | overflow   |
    +------------+
    */
    fn box_me(&self, msg: &str) -> String {
        let req_width = match msg.len() / self.width {
            0 => msg.len(),
            _ => self.width,
        };
        let n_lines = match msg.len() % req_width {
            0 => msg.len() / req_width,
            _ => (msg.len() / req_width) + 1,
        };

        let mut boundary = String::with_capacity(req_width + Handler::PADDING);
        boundary.push('+');
        for _ in 0..(boundary.capacity() - 3) {
            boundary.push('-');
        }
        boundary.push('+');

        /* text box + ascii art + ticks */
        let ret_sz = (boundary.capacity() + 1) * (3 + n_lines)
            + self.ascii_art.len() + 1
            + 7;
        let mut ret = String::with_capacity(ret_sz);

        ret.push_str("```\n");
        ret.push_str(boundary.as_str());
        ret.push('\n');

        for line in 0..n_lines-1 {
            let pos = line * req_width;
            ret.push_str("| ");
            ret.push_str(&msg[pos..pos + req_width]);
            ret.push_str(" |\n");
        }

        ret.push_str("| ");
        ret.push_str(&msg[(req_width * (n_lines - 1))..]);

        let pad_sz = match msg.len() % req_width {
            0 => 0,
            l => req_width - l,
        };

        for _ in 0..pad_sz {
            ret.push(' ');
        }
        ret.push_str(" |\n");

        ret.push_str(boundary.as_str());
        ret.push('\n');

        ret.push_str(self.ascii_art.as_str());
        ret.push_str("```");

        ret
    }
}

impl std::fmt::Display for SizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "you dumbass")
    }
}

impl error::Error for SizeError {}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("No Discord token defined in environment");

    let prefix ="!retf";
    let crab =
"      \\
       \\
         _~^~^~_
     \\) /  o o  \\ (/
       '_   -   _'
       / '-----' \\";

    let mut client = Client::new(&token, Handler::new(prefix, crab, 40).unwrap())
        .expect("Could not create client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
