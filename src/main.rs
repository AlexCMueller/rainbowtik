extern crate discord;

use discord::{Discord, State};
use discord::model::Event;

fn main()
{
    // Log in to discord
    // Token hard-coded for now, will pull from configuration file before initial commit
    // Or, perhaps I'll just replace the text will filler garbage before comitting
    let discord = Discord::from_bot_token("REDACTED")
        .expect("Failed to authenticate");

    // Establish web socket connection
    let (mut connection, ready) = discord.connect().expect("Connection Failed");

    let bot_state = State::new(ready);
    loop
    {
        match connection.recv_event()
        {
            Ok(Event::MessageCreate(message)) =>
            {
                // failsafe to keep bot from replying to itself
                if message.author.id == bot_state.user().id
                {
                    continue
                }

                // This line splits the message into arguments. 0 is command, 1 and on
                // is argument number
                let args: Vec<&str> = message.content.split_whitespace().collect();

                // This part is where commands are implemented!
                match args[0]
                {
                    "!test" => 
                    {
                        let _ = discord.send_message(&message.channel_id, "!test recieved", "", false);
                    }
                    _ => {}
                }
            }
            Ok(_) => {}
            Err(_) =>
            {
                break
            }
        }
    }
}
