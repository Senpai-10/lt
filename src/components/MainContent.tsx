import { T_Task, TasksDisplay } from '../types';
import { Task } from './Task';
import '../css/components/MainContent.css';
import EditIcon from '../assets/edit.svg';
import PlusIcon from '../assets/plus.svg';
import TrashIcon from '../assets/trash.svg';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api';

interface Props {
    tasksList: T_Task[];
    currentCategory: string | null;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    tasksSearchQuery: string;
    setTasksSearchQuery: React.Dispatch<React.SetStateAction<string>>;
    showTasks: TasksDisplay;
    setShowTasks: React.Dispatch<React.SetStateAction<TasksDisplay>>;
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
        showTasks,
        setShowTasks,
        getCategories,
        getTasks,
    } = props;
    const [newTask, setNewTask] = useState('');

    const handleAddTask = () => {
        if (newTask == '' || currentCategory == null) return;

        invoke('add_task', { title: newTask, category: currentCategory }).then(
            () => {
                getCategories();
                getTasks();
            },
        );

        setNewTask('');
    };

    const handleRemoveCategory = () => {
        if (currentCategory == null) return;

        invoke('remove_category', { name: currentCategory }).then(() => {
            setCurrentCategory(null);
            getCategories();
        });
    };

    return (
        <div className='main-content'>
            <div className='header'>
                <span className='category-name'>
                    {currentCategory != null ? currentCategory : 'All Tasks'}
                </span>
                <div className='header-options'>
                    <input
                        placeholder='Search'
                        value={tasksSearchQuery}
                        onChange={(e) =>
                            setTasksSearchQuery(e.currentTarget.value)
                        }
                    />
                    <select
                        value={showTasks}
                        title='Show tasks by status'
                        onChange={(e) =>
                            setShowTasks(e.currentTarget.value as TasksDisplay)
                        }
                    >
                        <option value='all'>all</option>
                        <option value='active'>active</option>
                        <option value='done'>done</option>
                    </select>
                    <img src={EditIcon} />
                    <img className="remove-category" onClick={handleRemoveCategory} src={TrashIcon} />
                </div>
            </div>
            <div className='sp'></div>
            <div className='tasks-list'>
                {tasksList.map((task) => {
                    return (
                        <Task
                            key={task.id}
                            task={task}
                            getCategories={getCategories}
                            getTasks={getTasks}
                        />
                    );
                })}
            </div>
            <div className="new-task">
                <input
                    className="new-task-input"
                    value={newTask}
                    onChange={(e) => setNewTask(e.currentTarget.value)}
                    onKeyDown={(e) => e.key == "Enter" ? handleAddTask() : null}
                    placeholder='Add a task'
                />
                <button onClick={handleAddTask}>
                    <img src={PlusIcon} />
                </button>
            </div>
        </div>
    );
}
