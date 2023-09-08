import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

import '../css/components/Popup.css';
import { T_TaskInRes } from '../types';

interface Props {
    task: T_TaskInRes;
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

    let created_at = new Date(task.created_at * 1000).toLocaleString();
    let updated_at = new Date(
        task.updated_at * 1000,
    ).toLocaleString();
    let done_at = new Date(
        (task.done_at || 0) * 1000,
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
                <span className='task-desc'>Created at: {created_at}</span>
                <span className='task-desc'>Last mod: {updated_at}</span>
                {task.done_at ? (
                    <span className='task-desc'>
                        Done at: {done_at}
                    </span>
                ) : null}
            </div>
            <div className='popup-close-detector' onClick={closePopup} />
        </>
    );
}
