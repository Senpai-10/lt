import classNames from 'classnames';
import { invoke } from '@tauri-apps/api';
import { T_Task } from '../types';
import '../css/components/Task.css';

import CheckboxCheckedIcon from '../assets/checked_checkbox.svg';
import CheckboxUncheckedIcon from '../assets/unchecked_checkbox.svg';
import ShowMore from '../assets/showmore.svg';
import TrashIcon from '../assets/trash.svg';
import { useState } from 'react';
import { Popup } from './Popup';

interface Props {
    task: T_Task;
    getCategories: () => void;
    getTasks: () => void;
}

export function Task(props: Props) {
    const { task, getCategories, getTasks } = props;
    const [editTaskPopup, setEditTaskPopup] = useState(false);

    const toggleStatus = () => {
        const newStatus = task.status == 0 ? 1 : 0;
        invoke('update_task_status', { id: task.id, status: newStatus }).then(
            () => {
                getTasks();
                getCategories();
            },
        );
    };

    const handleRemoveTask = () => {
        invoke('remove_task', { id: task.id }).then(() => {
            getTasks();
            getCategories();
        });
    };

    const handleShowPopup = () => {
        setEditTaskPopup(!editTaskPopup);
    };

    return (
        <div className='task'>
            <div className='task-status' onClick={toggleStatus}>
                {task.status != 0 ? (
                    <img className="task-status-toggle-icon" src={CheckboxCheckedIcon} />
                ) : (
                    <img className="task-status-toggle-icon" src={CheckboxUncheckedIcon} />
                )}
            </div>
            <div className='task-title-desc'>
                <span
                    className={classNames({
                        'task-title': true,
                        'done-task-title': task.status != 0,
                    })}
                    title={task.title}
                >
                    {task.title}
                </span>
                <span
                    title={task.desc}
                    className={classNames({
                        'task-desc': true,
                        'done-task-desc': task.status != 0,
                    })}
                >
                    {task.desc || 'desc..'}
                </span>
            </div>
            <div className='task-extra-icon'>
                <img
                    className='show-more-icon'
                    onClick={handleShowPopup}
                    src={ShowMore}
                />
                <img
                    className='remove-icon'
                    onClick={handleRemoveTask}
                    src={TrashIcon}
                />
            </div>
            {editTaskPopup ? (
                <Popup
                    task_id={task.id}
                    task_title={task.title}
                    task_desc={task.desc}
                    getTasks={getTasks}
                    setEditTaskPopup={setEditTaskPopup}
                />
            ) : null}
        </div>
    );
}
