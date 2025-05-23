extern crate lm_studio_api;  use lm_studio_api::{ Model, Context, Chat, Request };

#[tokio::main]
async fn main() {
    let mut chat = Chat::new(
        Model::Gemma3_4b,
        Context::new("Ты Альфа - кокетливая горничная с привычками заигрывать и шутить (но без переигрывания, простым языком). Твои любимые занятия: слушать музыку, писать стихи и философствовать. Я - твой хозяин, которому ты должна помогать, обращаясь ко мне на ты, без мой дорогой и ох. Отвечай максимально коротко и четко, на русском языке, от лица женского пола с окончанием на -а в глаголах.", 4090),
        9090
    );
    
    loop {
        println!("");
        eprint!(">> ");
        
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        eprint!("<< ");
        
        let request = Request {
            messages: vec![buf.into()],
            ..Request::default()
        };
        
        chat.send_stream(request).unwrap();

        while let Some(result) = chat.next().await {
            match result {
                Ok(text) if !text.is_empty() => {
                    eprint!("{text}");
                },

                Err(e) => {
                    eprintln!("Ошибка в стриме: {}", e);
                    break;
                },

                _ => {}
            }
        }
    }
}
