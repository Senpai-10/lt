import { useState, useEffect, useMemo, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

import { TasksDisplay, CategoriesData, T_TaskInRes } from './types';
import { Sidebar } from './components/Sidebar';
import { MainContent } from './components/MainContent';

import './css/components/App.css';

function App() {
    const [data, setData] = useState<T_TaskInRes[]>();
    const [categoriesData, setCategoriesData] = useState<CategoriesData>();
    const [category, setCategory] = useState<string | null>(null);
    const [tasksSearchQuery, setTasksSearchQuery] = useState('');
    const [showTasks, setShowTasks] = useState<TasksDisplay>('all');

    const addCategoryInputRef = useRef<HTMLInputElement>(null);
    const searchInputRef = useRef<HTMLInputElement>(null);
    const addTaskInputRef = useRef<HTMLInputElement>(null);

    const handleKeyPress = (event: KeyboardEvent) => {
        if (event.ctrlKey === true) {
            switch (event.key) {
                case 'q':
                    invoke('quit_app');
                    break;

                case 'f':
                    searchInputRef.current?.focus();
                    break;

                case 'c':
                    addCategoryInputRef.current?.focus();
                    break;

                case 't':
                    addTaskInputRef.current?.focus();
                    break;

                default:
                    break;
            }
        }
    };

    useEffect(() => {
        // attach the event listener
        document.addEventListener('keydown', handleKeyPress);

        // remove the event listener
        return () => {
            document.removeEventListener('keydown', handleKeyPress);
        };
    }, []);

    const filteredData = useMemo(() => {
        if (data == undefined) return [];
        return data.filter((task) => {
            if (showTasks === 'active' && task.status == 1) {
                return false;
            } else if (showTasks === 'done' && task.status == 0) {
                return false;
            }

            return task.title.toLowerCase().includes(tasksSearchQuery);
        });
    }, [data, tasksSearchQuery, showTasks]);

    const getTasks = () => {
        invoke('get_tasks', { category: category }).then((res: any) => {
            setData(res);
            console.log(res)
        });
    };

    const getCategories = () => {
        invoke('get_categories').then((data: any) => {
            setCategoriesData(data);
        });
    };

    useEffect(() => {
        getCategories();
    }, []);

    useEffect(() => {
        getTasks();
    }, [category]);

    if (data == undefined || categoriesData == undefined) {
        return;
    }

    return (
        <div className='container'>
            <Sidebar
                categoriesData={categoriesData}
                setCurrentCategory={setCategory}
                currentCategory={category}
                addCategoryInputRef={addCategoryInputRef}
                getCategories={getCategories}
            />
            <MainContent
                tasksList={filteredData}
                categoriesData={categoriesData}
                currentCategory={category}
                setCurrentCategory={setCategory}
                tasksSearchQuery={tasksSearchQuery}
                setTasksSearchQuery={setTasksSearchQuery}
                showTasks={showTasks}
                setShowTasks={setShowTasks}
                addTaskInputRef={addTaskInputRef}
                searchInputRef={searchInputRef}
                getCategories={getCategories}
                getTasks={getTasks}
            />
        </div>
    );
}

export default App;
