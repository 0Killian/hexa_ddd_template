use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct AppLayoutProps {
    pub title: String,
    pub current_menu_href: String,
    // pub user: AuthenticatedUser,
    pub children: Children,
}

#[function_component]
pub fn AppLayoutTemplate(props: &AppLayoutProps) -> Html {
    html! {
        <>
            {Html::from_html_unchecked(AttrValue::from("<!DOCTYPE html>"))}
            <html lang="en" class="h-full bg-gray-800">

            <head>
                <link href="/assets/main.css" rel="stylesheet" />
                <link rel="apple-touch-icon" sizes="180x180" href="/assets/apple-touch-icon.png"/>
                <link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon-32x32.png"/>
                <link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon-16x16.png"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link rel="preconnect" href="https://rsms.me/" />
                <link rel="stylesheet" href="https://rsms.me/inter/inter.css" />
                <title>{&props.title}</title>
            </head>

            <body hx-ext="response-targets">
                <main>{ props.children.clone() }</main>
                <script src="/assets/htmx.min.js"></script>
                <script src="/assets/htmx-response-target.min.js"></script>
                <script src="/assets/main.js"></script>
            </body>
        </html>
    </>
    }
}
