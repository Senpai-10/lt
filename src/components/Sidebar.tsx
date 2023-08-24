import { CategoriesData } from '../types';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api';

import { PlusIcon } from './icons/Plus';
import { SidebarItem } from './SidebarItem';

import '../css/components/Sidebar.css';

interface Props {
    currentCategory: string | null;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    categoriesData: CategoriesData;
    addCategoryInputRef: React.RefObject<HTMLInputElement>;
    getCategories: () => void;
}

export function Sidebar(props: Props) {
    const {
        categoriesData,
        getCategories,
        currentCategory,
        setCurrentCategory,
        addCategoryInputRef,
    } = props;

    const [newCategory, setNewCategory] = useState('');

    const handleAddCategory = () => {
        if (newCategory == '') return;

        invoke('add_category', { name: newCategory }).then(() => {
            setCurrentCategory(newCategory)
            getCategories();
        });

        setNewCategory('');
    };

    return (
        <div className='sidebar'>
            <SidebarItem
                name='All Tasks'
                type='all-tasks'
                is_active={currentCategory == null}
                total={{
                    tasks: categoriesData.total_tasks,
                    done: categoriesData.total_tasks_done,
                }}
                setCurrentCategory={setCurrentCategory}
            />

            <div className='sidebar-category-list'>
                {categoriesData.categories.map((x) => (
                    <SidebarItem
                        key={x.name}
                        name={x.name}
                        type='category-name'
                        total={{
                            tasks: x.total_tasks,
                            done: x.total_tasks_done,
                        }}
                        is_active={x.name == currentCategory}
                        setCurrentCategory={setCurrentCategory}
                    />
                ))}
            </div>

            <div className='sidebar-new-category-container'>
                <input
                    className='sidebar-new-category-input'
                    ref={addCategoryInputRef}
                    value={newCategory}
                    onChange={(e) => setNewCategory(e.currentTarget.value)}
                    onKeyDown={(e) =>
                        e.key == 'Enter' ? handleAddCategory() : null
                    }
                    placeholder='Add a category'
                />
                <button onClick={handleAddCategory}>
                    <PlusIcon />
                </button>
            </div>
        </div>
    );
}
