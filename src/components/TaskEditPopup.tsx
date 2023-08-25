import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

import '../css/components/Popup.css';
import { T_Task } from '../types';

interface Props {
    task: T_Task;
    trigger: React.Dispatch<React.SetStateAction<boolean>>;
    getTasks: () => void;
}

export function TaskEditPopup(props: Props) {
    const { task, trigger, getTasks } = props;

    const [taskTitle, setTaskTitle] = useState(task.title);
    const [taskDesc, setTaskDesc] = useState(task.desc);

    const closePopup = () => {
        trigger(false);

        if (taskTitle != task.title) {
            invoke('update_task_title', {
                id: task.id,
                title: taskTitle,
            });
            getTasks();
        }

        if (taskDesc != task.desc) {
            invoke('update_task_desc', { id: task.id, desc: taskDesc });
            getTasks();
        }
    };

    let creation_date = new Date(task.creation_date * 1000).toLocaleString();
    let modification_date = new Date(
        task.modification_date * 1000,
    ).toLocaleString();
    let completion_date = new Date(
        (task.completion_date || 0) * 1000,
    ).toLocaleString();

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
                <span className="task-desc">Creation: {creation_date}</span>
                <span className="task-desc">Last mod: {modification_date}</span>
                {task.completion_date ? (
                    <span className="task-desc">Completion: {completion_date}</span>
                ) : null}
            </div>
            <div className='popup-close-detector' onClick={closePopup}></div>
        </>
    );
}
