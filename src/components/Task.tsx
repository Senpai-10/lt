import classNames from 'classnames';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { T_Task } from '../types';
import '../css/components/Task.css';

interface Props {
    task: T_Task;
    getCategories: () => void;
    getTasks: () => void;
}

export function Task(props: Props) {
    const { task, getCategories, getTasks } = props;

    const [editingMode, setEditingMode] = useState<string | null>(null);
    const [editingModeText, setEditingModeText] = useState('');
    const dateLocale = Intl.DateTimeFormat().resolvedOptions().locale;

    const removeTask = (id: string) => {
        if (id == '') return;

        invoke('remove_task', { id: id });
        getCategories();
        getTasks();
    };

    const updateTaskStatus = (id: string, status: number) => {
        if (id == '') return;

        invoke('update_task_status', { id: id, status: status });
        getTasks();
        getCategories();
    };

    const updateTaskTitle = (id: string, title: string) => {
        invoke('update_task_title', { id: id, title: title });
    };

    const handleDoneEditing = (taskID: string) => {
        setEditingMode(null);
        updateTaskTitle(taskID, editingModeText);
        setEditingModeText('');
        getTasks();
    };

    const startEditingMode = (taskID: string, title: string) => {
        setEditingMode(taskID);
        setEditingModeText(title);
    };

    const updateTaskPriority = (
        e: React.ChangeEvent<HTMLInputElement>,
        taskID: string,
    ) => {
        invoke('update_task_priority', {
            id: taskID,
            newPriority: Number(e.currentTarget.value),
        });
        getTasks();
    };

    const isDone = task.status == 1 ? true : false;
    const newStatus = isDone ? 0 : 1;
    const completion_date = () => {
        if (task.completion_date == undefined) return;

        const date = new Date(task.completion_date * 1000);

        return (
            <span
                title={date.toLocaleString(dateLocale)}
                className='task-completion-date'
            >
                {date.toLocaleDateString(dateLocale)}
            </span>
        );
    };

    return (
        <div key={task.id} className={classNames({
            task: true,
            'priority-task': task.priority > 0 && task.status != 1
        })}>
            <div className='task-begin-container'>
                <div
                    className='drag-icon'
                    onDragStart={(e) =>
                        e.dataTransfer.setData('taskID', task.id)
                    }
                    draggable={true}
                ></div>
                <input
                    className='task-checkbox'
                    onChange={() => updateTaskStatus(task.id, newStatus)}
                    checked={isDone}
                    type='checkbox'
                />
            </div>
            {editingMode == task.id ? (
                <input
                    onKeyDown={(e) =>
                        e.code == 'Enter' ? handleDoneEditing(task.id) : null
                    }
                    placeholder='task title'
                    onChange={(e) => setEditingModeText(e.currentTarget.value)}
                    value={editingModeText}
                />
            ) : (
                <p
                    className={classNames({
                        'task-title': true,
                        'task-done': isDone
                    })}
                    onDoubleClick={() => startEditingMode(task.id, task.title)}
                >
                    {task.title}
                </p>
            )}
            <div className='task-extra'>
                {task.completion_date != undefined ? completion_date() : null}
                <input
                    className={classNames({
                        'task-priority-input': true,
                        'task-priority-input-disabled': isDone,
                    })}
                    onChange={(e) => updateTaskPriority(e, task.id)}
                    disabled={isDone}
                    value={task.priority}
                    type='number'
                />
                <button
                    className='remove-btn'
                    onClick={() => removeTask(task.id)}
                >
                    x
                </button>
            </div>
        </div>
    );
}
