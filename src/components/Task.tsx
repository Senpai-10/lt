import classNames from 'classnames';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api';

import { T_TaskInRes } from '../types';
import { TaskEditPopup } from './TaskEditPopup';
import { NewSubTaskPopup } from './NewSubTaskPopup';
import { CheckedCheckBoxIcon, UncheckedCheckBoxIcon } from './icons/Checkbox';
import { ShowMoreIcon } from './icons/ShowMore';
import { TrashIcon } from './icons/Trash';

import '../css/components/Task.css';
import { PlusIcon } from './icons/Plus';

interface Props {
    task: T_TaskInRes;
    indent: number,
    getCategories: () => void;
    getTasks: () => void;
}

export function Task(props: Props) {
    const { task, indent, getCategories, getTasks } = props;
    const [editTaskPopup, setEditTaskPopup] = useState(false);
    const [newSubTaskPopup, setNewSubTaskPopup] = useState(false);

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
        <>
            <div className='task' style={{ marginLeft: indent * 25 }}>
                <div className='task-status' onClick={toggleStatus}>
                    {task.status != 0 ? (
                        <CheckedCheckBoxIcon />
                    ) : (
                        <UncheckedCheckBoxIcon />
                    )}
                </div>
                <div className='task-text-container'>
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
                        {task.desc}
                    </span>
                </div>
                <div className='task-extra-icon'>
                    <div onClick={handleShowPopup}>
                        <ShowMoreIcon />
                    </div>
                    <div onClick={() => setNewSubTaskPopup(!newSubTaskPopup)}>
                        <PlusIcon />
                    </div>
                    <div onClick={handleRemoveTask}>
                        <TrashIcon />
                    </div>
                </div>
                {editTaskPopup ? (
                    <TaskEditPopup
                        task={task}
                        getTasks={getTasks}
                        trigger={setEditTaskPopup}
                    />
                ) : null}
                {newSubTaskPopup ? (
                    <NewSubTaskPopup
                        parentID={task.id}
                        currentCategory={task.category_name}
                        getCategories={getCategories}
                        getTasks={getTasks}
                        trigger={setNewSubTaskPopup}
                    />
                ) : null}
            </div>
            {
                task.sub_tasks.map((subtask) => <Task key={subtask.id} task={subtask} indent={indent + 1} getTasks={getTasks} getCategories={getCategories} />)
            }
        </>
    );
}
