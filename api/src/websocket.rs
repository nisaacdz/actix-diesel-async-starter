// use actix_web::{Error, HttpRequest, HttpResponse, web};
// use actix_ws::Message;
// use futures_util::StreamExt;
// use infra::services::websocket::WebSocketService;
// use std::rc::Rc;
// use tokio::sync::mpsc;

// pub async fn ws_handler(
//     req: HttpRequest,
//     user: web::ReqData<Rc<AuthenticatedUser>>,
//     socket_service: web::Data<WebSocketService>,
//     stream: web::Payload,
// ) -> Result<HttpResponse, Error> {
//     let (response, mut session_ws, mut msg_stream) = actix_ws::handle(&req, stream)?;

//     let (tx, mut rx) = mpsc::unbounded_channel::<String>();

//     actix_web::rt::spawn(async move {
//         loop {
//             tokio::select! {
//                 Some(Ok(msg)) = msg_stream.next() => {
//                     match msg {
//                         Message::Text(text) => {
//                             tracing::debug!("Received ws message: {}", text);
//                             // We won't need to receive events from the client yet
//                         }
//                         Message::Close(_) => {
//                             tracing::info!("WebSocket closed by client");
//                             break;
//                         }
//                         Message::Ping(bytes) => {
//                             tracing::debug!("Received ping");
//                             if let Err(e) = session_ws.pong(&bytes).await {
//                                 tracing::error!("Error sending pong: {}", e);
//                                 break;
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//                 Some(text) = rx.recv() => {
//                     if let Err(e) = session_ws.text(text).await {
//                         tracing::error!("Error sending message to WebSocket: {}", e);
//                         break;
//                     }
//                 }
//                 else => break,
//             }
//         }

//         socket_service.remove_user(&user.id);
//     });

//     Ok(response)
// }
