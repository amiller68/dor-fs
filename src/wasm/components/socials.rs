use leptos::*;

#[component]
pub fn Socials() -> impl IntoView {
    view! {
        <div class="socials">
            <a href="https://github.com/amiller68" class="icon">
                <img src="/static/icons/github.svg" alt="Github" class="icon"/>
            </a>
            <a href="https://twitter.com/lord_krondor" class="icon">
                <img src="/static/icons/twitter.svg" alt="Twitter" class="icon"/>
            </a>
            <a href="mailto:al@krondor.org" class="icon">
                <img src="/static/icons/email.svg" alt="Email" class="icon"/>
            </a>
            <a href="tg://resolve?domain=lord_krondor" class="icon">
                <img src="/static/icons/telegram.svg" alt="Telegram" class="icon"/>
            </a>
            <a href="https://www.linkedin.com/in/al-miller-110953171/" class="icon">
                <img src="/static/icons/linkedin.svg" alt="LinkedIn" class="icon"/>
            </a>
        </div>
    }
}
