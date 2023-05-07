use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write, self};


#[derive(Serialize, Deserialize, Debug)]
struct Message{
    src: String,
    dest: String,
    body: Body
}

#[derive(Serialize, Deserialize, Debug)]
struct Body{
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload{
    Echo{
        echo: String
    },
    EchoOk {
        echo: String
    },
    Init {
        node_id: String,
        node_ids: Vec<String>
    },
    InitOk,
}

struct Node{
    id: usize
}

impl Node{
    fn listen(&mut self, input: Message, output: &mut StdoutLock){
        match input.body.payload{
            Payload::Init { node_id, node_ids } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::InitOk,
                    },
                };

                serde_json::to_writer(&mut *output, &reply).unwrap();
                output.write_all(b"\n").unwrap();

            },
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply).unwrap();
                output.write_all(b"\n").unwrap();
                self.id += 1;

            },
            Payload::EchoOk { .. } => {},
            Payload::InitOk { .. } => {}

        }
    }
}

fn main() -> io::Result<()>{
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut stdout = std::io::stdout().lock();
    
    let mut node = Node { id: 0 };

    for input in inputs{
        node.listen(input.expect("failed unpacking data"), &mut stdout)

    }
    Ok(())
}
