import { useState } from 'react';
import classNames from 'classnames';
import { invoke } from '@tauri-apps/api/tauri';
import { CategoriesData } from '../types';
import '../css/components/Sidebar.css';

interface Props {
    currentCategory: string | null;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    categoriesData: CategoriesData;
    getCategories: () => void;
    getTasks: () => void;
}

export function Sidebar(props: Props) {
    const {
        categoriesData,
        currentCategory,
        setCurrentCategory,
        getCategories,
        getTasks,
    } = props;
    const [newCategoryInput, setNewCategoryInput] = useState('');

    const addCategory = () => {
        if (newCategoryInput == '') return;

        invoke('add_category', { name: newCategoryInput });
        setNewCategoryInput('');
        setCurrentCategory(newCategoryInput);
        getCategories();
    };

    const handleOnDrop = (e: React.DragEvent, category_name: string) => {
        if (category_name == currentCategory || currentCategory == null) return;

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
        <div className='side-bar'>
            <div
                className={classNames({
                    'category-tasks-all-done':
                        categoriesData.total_tasks_done ==
                        categoriesData.total_tasks &&
                        categoriesData.total_tasks != 0,
                    category: true,
                    'current-category': currentCategory == null,
                })}
                onClick={() => setCurrentCategory(null)}
            >
                <span>All</span>
                <span className='category-tasks-count'>
                    {categoriesData.total_tasks_done}/
                    {categoriesData.total_tasks}
                </span>
            </div>
            <div className='new-category-container'>
                <input
                    className='new-category-input'
                    onKeyDown={(e) =>
                        e.code == 'Enter' ? addCategory() : null
                    }
                    onChange={(e) => setNewCategoryInput(e.currentTarget.value)}
                    value={newCategoryInput}
                    placeholder='category..'
                />
                <button onClick={addCategory} style={{ fontWeight: 'bold' }}>
                    +
                </button>
            </div>
            {categoriesData.categories.map((x) => (
                <div
                    className={classNames({
                        'category-tasks-all-done':
                            x.total_tasks_done == x.total_tasks &&
                            x.total_tasks != 0,
                        category: true,
                        'current-category': currentCategory == x.name,
                    })}
                    key={x.name}
                    onClick={() => setCurrentCategory(x.name)}
                    onDrop={(e) => handleOnDrop(e, x.name)}
                    onDragOver={(e) => e.preventDefault()}
                >
                    <span>{x.name}</span>
                    <span className='category-tasks-count'>
                        {x.total_tasks_done}/{x.total_tasks}
                    </span>
                </div>
            ))}
        </div>
    );
}
