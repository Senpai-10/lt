import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';

import { loadTheme } from '../helpers';

import '../css/components/Popup.css';
import '../css/components/SettingsPopup.css';

interface Props {
    trigger: React.Dispatch<React.SetStateAction<boolean>>;
}

export function SettingsPopup(props: Props) {
    const closePopup = () => {
        props.trigger(false);
    };

    const [themes, setThemes] = useState<string[]>();
    const [currentTheme, setCurrentTheme] = useState<string>();
    const [selectedTheme, setSelectedTheme] = useState<string>();

    useEffect(() => {
        invoke('get_available_themes').then((data: any) => {
            setThemes(data);
        });
        invoke('get_current_theme').then((v: any) => {
            setCurrentTheme(v);
            setSelectedTheme(v);
        });
    }, []);

    useEffect(() => {
        if (selectedTheme) {
            loadTheme(selectedTheme);
        }
    }, [selectedTheme]);

    if (themes == undefined) {
        return <></>;
    }

    const saveSettings = () => {
        if (selectedTheme != currentTheme) {
            invoke('set_theme', { newTheme: selectedTheme });
        }
    };

    return (
        <>
            <div className='popup'>
                <div className='popup-settings-options'>
                    <span>Theme</span>
                    <select
                        value={selectedTheme}
                        onChange={(e) =>
                            setSelectedTheme(e.currentTarget.value)
                        }
                    >
                        {themes.map((theme) => (
                            <option key={theme} value={theme}>
                                {theme}
                            </option>
                        ))}
                    </select>
                </div>
                <button
                    onClick={saveSettings}
                    className='popup-save-settings-btn'
                >
                    Save
                </button>
            </div>
            <div className='popup-close-detector' onClick={closePopup}></div>
        </>
    );
}
