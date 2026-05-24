use askama::Template; 
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response}; 

#[derive(Template)] 
#[template(path = "index.html")] 
pub struct DashboardTemplate {
    pub running: bool,
}

#[derive(Template)]
#[template(path = "status_box.html")]
pub struct StatusBoxTemplate {
    pub running: bool,
}

pub struct HtmlTemplate<T>(pub T); 

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template, 
{
    fn into_response(self) -> Response {
        match self.0.render() {
            // If rendering is successful, return an HTML response
            Ok(html) => Html(html).into_response(), 
            // If rendering fails, return an internal server error
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}