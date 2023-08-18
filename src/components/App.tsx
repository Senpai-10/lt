import { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { CategoriesData, T_Task } from '../types';
import '../css/components/App.css';
import { Sidebar } from './Sidebar';
import { MainContent } from './MainContent';

function App() {
    const [data, setData] = useState<T_Task[]>();
    const [categoriesData, setCategories] = useState<CategoriesData>();
    const [category, setCategory] = useState<string | null>(null);
    const [tasksSearchQuery, setTasksSearchQuery] = useState('');
    const [hideDone, setHideDone] = useState(false);

    const filteredData = useMemo(() => {
        if (data == undefined) return [];
        return data.filter((task) => {
            if (hideDone === true && task.status == 1) {
                return false;
            }
            return task.title.toLowerCase().includes(tasksSearchQuery);
        });
    }, [data, tasksSearchQuery, hideDone]);

    const getTasks = () => {
        invoke('get_tasks', { category: category }).then((res: any) => {
            setData(res);
        });
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
        return;
    }

    return (
        <div className='container'>
            <Sidebar
                categoriesData={categoriesData}
                currentCategory={category}
                setCurrentCategory={setCategory}
                getCategories={getCategories}
                getTasks={getTasks}
            />
            <MainContent
                tasksList={filteredData}
                currentCategory={category}
                setCurrentCategory={setCategory}
                tasksSearchQuery={tasksSearchQuery}
                setTasksSearchQuery={setTasksSearchQuery}
                setHideDone={setHideDone}
                hideDone={hideDone}
                getCategories={getCategories}
                getTasks={getTasks}
            />
        </div>
    );
}

export default App;
