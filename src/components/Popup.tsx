import { invoke } from "@tauri-apps/api";
import { useState } from "react";

interface Props {
    task_id: string;
    task_title: string;
    task_desc?: string;
    setEditTaskPopup: React.Dispatch<React.SetStateAction<boolean>>;
    getTasks: () => void
}

export function Popup(props: Props) {
    const [taskTitle, setTaskTitle] = useState(props.task_title)
    const [taskDesc, setTaskDesc] = useState(props.task_desc)

    const closePopup = () => {
        props.setEditTaskPopup(false)

        if (taskTitle != props.task_title) {
            invoke('update_task_title', { id: props.task_id, title: taskTitle })
            props.getTasks()
        }

        if (taskDesc != props.task_desc) {
            invoke('update_task_desc', { id: props.task_id, desc: taskDesc })
            props.getTasks()
        }
    }

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
                    className='popup-task-desc'
                    placeholder='Task Description'
                    onChange={(e) => setTaskDesc(e.currentTarget.value)}
                    value={taskDesc}
                />
            </div>
            <div
                className='popup-close-detector'
                onClick={closePopup}
            ></div>
        </>
    );
}
