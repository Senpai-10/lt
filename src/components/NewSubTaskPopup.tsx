import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

import '../css/components/Popup.css';

interface Props {
    parentID: string;
    currentCategory: string | null
    trigger: React.Dispatch<React.SetStateAction<boolean>>;
    getTasks: () => void;
    getCategories: () => void;
}

export function NewSubTaskPopup(props: Props) {
    const { parentID, currentCategory, trigger, getTasks, getCategories } = props;

    const [taskTitle, setTaskTitle] = useState('');
    const [taskDesc, setTaskDesc] = useState('');

    const closePopup = () => {
        trigger(false);

        invoke('add_subtask', {
            parentId: parentID,
            title: taskTitle,
            category: currentCategory
        });

        getTasks();
        getCategories();
    };

    return (
        <>
            <div className='popup'>
                <h4>Title</h4>
                <input
                    placeholder='Task Title'
                    onChange={(e) => setTaskTitle(e.currentTarget.value)}
                    value={taskTitle}
                />
                <h4>Description</h4>
                <textarea
                    style={{ flex: 1 }}
                    placeholder='Task Description'
                    onChange={(e) => setTaskDesc(e.currentTarget.value)}
                    value={taskDesc}
                />
                <button onClick={closePopup}>Save</button>
            </div>
            <div className='popup-close-detector' onClick={closePopup} />
        </>
    );
}
