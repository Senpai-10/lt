import { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { TasksDisplay, CategoriesData, T_Task } from '../types';
import '../css/components/App.css';
import { Sidebar } from './Sidebar';
import { MainContent } from './MainContent';

function App() {
    const [data, setData] = useState<T_Task[]>();
    const [categoriesData, setCategoriesData] = useState<CategoriesData>();
    const [category, setCategory] = useState<string | null>(null);
    const [tasksSearchQuery, setTasksSearchQuery] = useState('');
    const [showTasks, setShowTasks] = useState<TasksDisplay>('all');

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
                getCategories={getCategories}
            />
            <MainContent
                tasksList={filteredData}
                currentCategory={category}
                setCurrentCategory={setCategory}
                tasksSearchQuery={tasksSearchQuery}
                setTasksSearchQuery={setTasksSearchQuery}
                showTasks={showTasks}
                setShowTasks={setShowTasks}
                getCategories={getCategories}
                getTasks={getTasks}
            />
        </div>
    );
}

export default App;
