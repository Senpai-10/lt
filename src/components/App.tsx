import { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { CategoriesData, Task } from '../types';
import '../css/components/app.css';
import { Sidebar } from './sidebar';
import { MainContent } from './main_content';

function App() {
    const [data, setData] = useState<Task[]>();
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
