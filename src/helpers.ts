import { invoke } from '@tauri-apps/api';

function getCustomThemeContainer(): HTMLElement {
    const id = 'custom-theme';
    let element = document.getElementById(id);

    if (element == null) {
        let newele = document.createElement('style');
        newele.setAttribute('id', id);
        document.head.appendChild(newele);
        return newele;
    }

    return element;
}

export function loadTheme(theme: string | null) {
    invoke('get_theme_css', { targetTheme: theme }).then((css: any) => {
        // Inject css
        const themeContainer = getCustomThemeContainer();

        themeContainer.innerHTML = css;
    });
}
