use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::HttpResponse;

use crate::handlers::{AppError, AppResponseErrorBody};

/// actix-webが、ハンドラがない場合やエクストラクタで発生したエラーなどをJSON形式に変換するミドルウェア
pub fn default_error_handler<B>(
    mut service_response: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>>
where
    B: actix_web::body::MessageBody,
    <B as MessageBody>::Error: std::fmt::Debug,
{
    // Content-Typeがapplication/jsonの場合はそのまま返す
    let content_type = service_response.headers().get(header::CONTENT_TYPE);
    if content_type.is_some() && content_type.unwrap() == "application/json" {
        return Ok(ErrorHandlerResponse::Response(
            service_response.map_into_left_body(),
        ));
    }

    // レスポンスヘッダーにContent-Typeを追加して、リクエストとレスポンスに分離
    service_response.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    let status_code = service_response.response().status();
    let (request, response) = service_response.into_parts();
    // actix-webが返すエラーレスポンスのボディは短いため同期処理
    let body_bytes =
        futures::executor::block_on(actix_http::body::to_bytes(response.into_body())).unwrap();
    // ボディを文字列に変換
    let body = std::str::from_utf8(&body_bytes).unwrap_or("Something wrong with me");
    // レスポンスを作成
    let body = AppResponseErrorBody {
        status_code,
        app_error: AppError::None,
        message: body.to_string(),
    };
    let response = HttpResponse::build(status_code).body(serde_json::to_string(&body).unwrap());
    let response = ServiceResponse::new(request, response)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(response))
}
