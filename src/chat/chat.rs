use crate::prelude::*;
use super::*;

use reqwest::Client;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

// The LM Studio chat
pub struct Chat {
    model: Model,
    context: Context,
    url: String,
    client: Client,
    reader: Option<ResponseReader>,
}

impl Chat {
    // Creates new chat
    pub fn new<M, C>(model: M, context: C, port: usize) -> Self
    where
        M: Into<Model>,
        C: Into<Context>
    {
        Self {
            model: model.into(),
            context: context.into(),
            url: fmt!("http://127.0.0.1:{port}/v1/chat/completions"),
            client: Client::new(),
            reader: None
        }
    }

    // Send request to chat
    pub async fn send(&mut self, mut request: Request) -> Result<Response> {
        // init request:
        for msg in request.messages {
            self.context.add(msg);
        }

        request.messages = self.context.get();
        
        if let Model::Custom(s) = &request.model && s.is_empty() {
            request.model = self.model.clone();
        }

        request.stream = false;

        // send request:
        let response = self.client.post(&self.url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<Response>()
            .await?;

        // add answer to chat context:
        if let Some(choice) = response.choices.get(0) {
            let answer = Message::new(Role::Assistant, choice.message.content.clone());
            self.context.add(answer);
        }

        Ok(response)
    }

    // Send request to chat as stream
    pub fn send_stream(&mut self, mut request: Request) -> Result<()> {
        // init request:
        for msg in request.messages {
            self.context.add(msg);
        }

        request.messages = self.context.get();
        
        if let Model::Custom(s) = &request.model && s.is_empty() {
            request.model = self.model.clone();
        }

        request.stream = true;

        // Создаём канал для передачи результатов из потока
        let (tx, rx) = mpsc::unbounded_channel();

        // Клонируем client и url для async задачи
        let client = self.client.clone();
        let url = self.url.clone();
        let req_clone = request.clone();

        // Запускаем асинхронную задачу для чтения потока
        tokio::spawn(async move {
            let res = client.post(&url)
                .json(&req_clone)
                .send()
                .await;

            match res {
                Ok(response) => {
                    let mut stream = response.bytes_stream();

                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(chunk) => {
                                let chunk_str = String::from_utf8_lossy(&chunk);

                                for line in chunk_str.lines() {
                                    if line.starts_with("data: ") {
                                        let data = &line[6..];
                                        if data == "[DONE]" {
                                            break;
                                        }

                                        let sr: Result<Stream> = serde_json::from_str(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
                                        if let Ok(sr) = sr {
                                            for choice in sr.choices {
                                                // Отправляем choice в канал
                                                if tx.send(Ok(choice)).is_err() {
                                                    // Получатель закрылся
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(Err(e));
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e));
                }
            }
        });

        self.reader = Some(ResponseReader::new(UnboundedReceiverStream::new(rx)));

        Ok(())
    }

    // Read next stream choice
    pub async fn next(&mut self) -> Option<std::result::Result<String, reqwest::Error>> {
        if let Some(reader) = &mut self.reader {
            let result = reader.next().await;

            if reader.is_ready {
                self.context.add(reader.message.clone())
            }
            
            result
        } else {
            None
        }
    }
}
