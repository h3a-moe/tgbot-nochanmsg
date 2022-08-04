use anyhow::anyhow;
use teloxide::{dispatching::DispatcherHandlerRx, prelude::*};
use tokio_stream::wrappers::UnboundedReceiverStream;

type MessageEvent = UpdateWithCx<AutoSend<Bot>, Message>;
type Result<T> = anyhow::Result<T>;

async fn on_message(event: &MessageEvent) -> Result<()> {
    let MessageEvent {
        requester: bot,
        update: message,
    } = event;

    let sender_chat = message
        .sender_chat()
        .ok_or_else(|| anyhow!("No sender chat"))?;

    // anonymous admin message
    if message.chat.id == sender_chat.id {
        return Ok(());
    }

    let channel_id = bot.get_chat(message.chat.id).await?.linked_chat_id();

    // linked channel message
    if channel_id == Some(sender_chat.id) {
        return Ok(());
    }

    if let Err(_) = event.delete_message().await {
        let _ = bot
            .send_message(
                message.chat_id(),
                "Detected a channel message that is not from the linked channel, but failed to delete it.\n\n\
                 Did you give me permission to delete messages?",
            )
            .await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();

    Dispatcher::new(bot)
        .messages_handler(|event: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(event).for_each_concurrent(None, |event| async move {
                let _ = on_message(&event).await;
                let _ = respond(());
            })
        })
        .dispatch()
        .await;
}
