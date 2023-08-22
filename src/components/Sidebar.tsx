import { CategoriesData } from '../types';
import { invoke } from '@tauri-apps/api';
import PlusIcon from '../assets/plus.svg';
import '../css/components/Sidebar.css';
import { SidebarItem } from './SidebarItem';
import { useEffect, useRef, useState } from 'react';

interface Props {
    currentCategory: string | null;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    categoriesData: CategoriesData;
    getCategories: () => void;
}

export function Sidebar(props: Props) {
    const {
        categoriesData,
        getCategories,
        currentCategory,
        setCurrentCategory,
    } = props;

    const [newCategory, setNewCategory] = useState('');
    const addCategoryInput = useRef<HTMLInputElement>(null);

    const handleAddCategory = () => {
        if (newCategory == '') return;

        invoke('add_category', { name: newCategory }).then(() => {
            setCurrentCategory(newCategory)
            getCategories();
        });

        setNewCategory('');
    };

    const handleKeyPress = (event: KeyboardEvent) => {
        if (event.ctrlKey === true && event.key == 'c' && addCategoryInput != null) {
            addCategoryInput.current?.focus()
        }
    }

    useEffect(() => {
        // attach the event listener
        document.addEventListener('keydown', handleKeyPress);

        // remove the event listener
        return () => {
            document.removeEventListener('keydown', handleKeyPress);
        };
    }, []);

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

            <div className='category-list'>
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

            <div className='sidebar-new-category'>
                <input
                    className='sidebar-new-category-input'
                    ref={addCategoryInput}
                    value={newCategory}
                    onChange={(e) => setNewCategory(e.currentTarget.value)}
                    onKeyDown={(e) =>
                        e.key == 'Enter' ? handleAddCategory() : null
                    }
                    placeholder='Add a category'
                />
                <button onClick={handleAddCategory}>
                    <img src={PlusIcon} />
                </button>
            </div>
        </div>
    );
}
