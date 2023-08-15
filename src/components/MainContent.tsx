import { useState } from 'react';
import classNames from 'classnames';
import { invoke } from '@tauri-apps/api/tauri';
import { Task } from "../types"
import { Navbar } from './Navbar';
import '../css/components/MainContent.css';

interface Props {
    tasksList: Task[]
    currentCategory: string | null;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    tasksSearchQuery: string
    setTasksSearchQuery: React.Dispatch<React.SetStateAction<string>>
    hideDone: boolean
    setHideDone: React.Dispatch<React.SetStateAction<boolean>>
    getCategories: () => void;
    getTasks: () => void;
}

export function MainContent(props: Props) {
    const {
        tasksList,
        currentCategory,
        setCurrentCategory,
        tasksSearchQuery,
        setTasksSearchQuery,
        hideDone,
        setHideDone,
        getCategories,
        getTasks,
    } = props;
    const [editingMode, setEditingMode] = useState<string | null>(null);
    const [editingModeText, setEditingModeText] = useState('');
    const [newTaskInput, setNewTaskInput] = useState('');
    const dateLocale = Intl.DateTimeFormat().resolvedOptions().locale;

    const addTask = () => {
        if (newTaskInput == '') return;

        invoke('add_task', {
            title: newTaskInput,
            category: currentCategory == null ? 'main' : currentCategory,
        });
        setNewTaskInput('');
        getCategories();
        getTasks();
    };


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

    return (
        <div className='main-content'>
            <Navbar
                currentCategory={currentCategory}
                tasksSearchQuery={tasksSearchQuery}
                hideDone={hideDone}
                setCurrentCategory={setCurrentCategory}
                setTasksSearchQuery={setTasksSearchQuery}
                setHideDone={setHideDone}
                getCategories={getCategories}
            />
            {tasksList.length == 0 && currentCategory == null ? (
                <div>
                    <p>empty list</p>
                </div>
            ) : (
                tasksList.map((task: Task) => {
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
                        <div key={task.id} className='task'>
                            <div className='task-begin-container'>
                                <div
                                    className='drag-icon'
                                    onDragStart={(e) =>
                                        e.dataTransfer.setData(
                                            'taskID',
                                            task.id,
                                        )
                                    }
                                    draggable={true}
                                ></div>
                                <input
                                    className='task-checkbox'
                                    onChange={() =>
                                        updateTaskStatus(task.id, newStatus)
                                    }
                                    checked={isDone}
                                    type='checkbox'
                                />
                            </div>
                            {editingMode == task.id ? (
                                <input
                                    onKeyDown={(e) =>
                                        e.code == 'Enter'
                                            ? handleDoneEditing(task.id)
                                            : null
                                    }
                                    placeholder='task title'
                                    onChange={(e) =>
                                        setEditingModeText(
                                            e.currentTarget.value,
                                        )
                                    }
                                    value={editingModeText}
                                />
                            ) : (
                                <p
                                    className={classNames({
                                        'task-title': true,
                                        'task-done': isDone,
                                        'priority-task':
                                            task.priority > 0 &&
                                            task.status != 1,
                                    })}
                                    onDoubleClick={() =>
                                        startEditingMode(
                                            task.id,
                                            task.title,
                                        )
                                    }
                                >
                                    {task.title}
                                </p>
                            )}
                            <div className='task-extra'>
                                {task.completion_date != undefined
                                    ? completion_date()
                                    : null}
                                <input
                                    className={classNames({
                                        'task-priority-input': true,
                                        'task-priority-input-disabled':
                                            isDone,
                                    })}
                                    onChange={(e) =>
                                        updateTaskPriority(e, task.id)
                                    }
                                    disabled={isDone}
                                    value={task.priority}
                                    type='number'
                                />
                                <button
                                    className='remove-btn'
                                    onClick={() => removeTask(task.id)}
                                >
                                    Del
                                </button>
                            </div>
                        </div>
                    );
                })
            )}
            {currentCategory != null ? (
                <>
                    <input
                        onKeyDown={(e) =>
                            e.code == 'Enter' ? addTask() : null
                        }
                        onChange={(e) =>
                            setNewTaskInput(e.currentTarget.value)
                        }
                        value={newTaskInput}
                        placeholder='task..'
                    />
                    {/*<button onClick={addTask}>add</button>*/}
                </>
            ) : null}
        </div>
    )
}

