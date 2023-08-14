import React, { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { CategoriesData, Task } from './types';
import classNames from 'classnames';
import './App.css';

function App() {
    const [data, setData] = useState<Task[]>();
    const [categoriesData, setCategories] = useState<CategoriesData>();
    const [category, setCategory] = useState<string | null>(null);
    const [newTaskInput, setNewTaskInput] = useState('');
    const [newCategoryInput, setNewCategoryInput] = useState('');
    const [tasksSearchQuery, setTasksSearchQuery] = useState('');

    const filteredData = useMemo(() => {
        if (data == undefined) return []
        return data.filter((task) => {
            return task.title.toLowerCase().includes(tasksSearchQuery)
        })
    }, [data, tasksSearchQuery])

    const getTasks = () => {
        if (category != null) {
            invoke('get_tasks', { category: category }).then((res: any) => {
                setData(res);
            });
        } else {
            invoke('get_tasks').then((res: any) => {
                setData(res);
            });
        }
    };

    const getCategories = () => {
        invoke('get_categories').then((data: any) => {
            setCategories(data);
        });
    };

    useEffect(() => {
        getCategories();
    }, []);

    useEffect(() => {
        getTasks();
    }, [category]);

    if (data == undefined || categoriesData == undefined) {
        return <h1>Loading</h1>;
    }

    const addCategory = () => {
        if (newCategoryInput == '') return;

        invoke('add_category', { name: newCategoryInput });
        setNewCategoryInput('');
        setCategory(newCategoryInput);
        getCategories();
    };

    const addTask = () => {
        if (newTaskInput == '') return;

        invoke('add_task', {
            title: newTaskInput,
            category: category == null ? 'main' : category,
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
    };

    const updateTaskTitle = (id: string, title: string) => {
        invoke('update_task_title', { id: id, title: title });
    };

    const handleOnDrop = (e: React.DragEvent, category_name: string) => {
        if (category_name == category || category == null) return;

        let taskID = e.dataTransfer.getData('taskID') as string;

        console.log(`moved task: '${taskID}' -> category: '${category_name}'`);
        invoke('update_task_category', {
            id: taskID,
            newCategory: category_name,
        });
        getCategories();
        getTasks();
    };

    return (
        <div className='container'>
            <div className='side-bar'>
                <div
                    className={classNames({
                        'all-category': true,
                        category: true,
                        'current-category': category == null,
                    })}
                    onClick={() => setCategory(null)}
                >
                    <span>All</span>
                    <span className='category-tasks-count'>{categoriesData.total_tasks}</span>
                </div>
                {categoriesData.categories.map((x) => (
                    <div
                        className={classNames({
                            category: true,
                            'current-category': category == x.name,
                        })}
                        key={x.name}
                        onClick={() => setCategory(x.name)}
                        onDrop={(e) => handleOnDrop(e, x.name)}
                        onDragOver={(e) => e.preventDefault()}
                    >
                        <span>{x.name}</span>
                        <span className='category-tasks-count'>{x.total_tasks}</span>
                    </div>
                ))}
                <div className='new-category-container'>
                    <input
                        className='new-category-input'
                        onKeyDown={(e) =>
                            e.code == 'Enter' ? addCategory() : null
                        }
                        onChange={(e) =>
                            setNewCategoryInput(e.currentTarget.value)
                        }
                        value={newCategoryInput}
                        placeholder='category..'
                    />
                    <button
                        onClick={addCategory}
                        style={{ fontWeight: 'bold' }}
                    >
                        +
                    </button>
                </div>
            </div>
            <div className='main-content'>
                <input placeholder='search' value={tasksSearchQuery} onChange={(e) => setTasksSearchQuery(e.currentTarget.value)} />
                {data.length == 0 && category == null ? (
                    <div>
                        <p>empty list</p>
                    </div>
                ) : (
                    filteredData.map((task: Task) => {
                        const isDone = task.status == 1 ? true : false;
                        const newStatus = isDone ? 0 : 1;

                        return (
                            <div key={task.id} className='task'>
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
                                <p
                                    contentEditable={true}
                                    suppressContentEditableWarning={true}
                                    className={isDone ? 'task-done' : ''}
                                    onInput={(e) =>
                                        updateTaskTitle(
                                            task.id,
                                            e.currentTarget.textContent
                                                ? e.currentTarget.textContent
                                                : '',
                                        )
                                    }
                                >
                                    {task.title}
                                </p>
                                <div className='task-extra'>
                                    <button
                                        className='remove-task-btn'
                                        onClick={() => removeTask(task.id)}
                                    >
                                        Del
                                    </button>
                                </div>
                            </div>
                        );
                    })
                )}
                {category != null ? (
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
        </div>
    );
}

export default App;
