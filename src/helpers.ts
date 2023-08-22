import { invoke } from "@tauri-apps/api";

function initCustomStylesTag() {
    let ele = document.getElementById("custom-styles")

    if (ele == null) {
        let newele = document.createElement("style")
        newele.setAttribute('id', 'custom-styles')
        document.head.appendChild(newele);
    }
}

export function loadTheme(theme: string) {
    invoke('get_theme_css', { targetTheme: theme }).then((v: any) => {
        initCustomStylesTag();

        let c = document.getElementById("custom-styles");

        if (c != null) {
            c.innerText = v;
        }
    })
}

export function loadCurrentTheme() {
    invoke('get_current_theme_css').then((v: any) => {
        initCustomStylesTag();

        let c = document.getElementById("custom-styles");

        if (c != null) {
            c.innerText = v;
        }
    })
}
