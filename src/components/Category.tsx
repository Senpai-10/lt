import { invoke } from '@tauri-apps/api';
import classNames from 'classnames';
import { T_Category } from "../types"
import "../css/components/Category.css"

interface Props {
    label: string
    category: T_Category
    currentCategory: string | null
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    getCategories: () => void;
    getTasks: () => void;
}

export function Category(props: Props) {
    const { label, category, currentCategory, setCurrentCategory, getCategories, getTasks } = props;

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
        <div
            className={classNames({
                'category-tasks-all-done':
                    category.total_tasks_done == category.total_tasks &&
                    category.total_tasks != 0,
                category: true,
                'current-category': currentCategory == category.name,
            })}
            onClick={() => setCurrentCategory(category.name)}
            onDrop={(e) => handleOnDrop(e, category.name)}
            onDragOver={(e) => e.preventDefault()}
        >
            <span>{label}</span>
            <span className='category-tasks-count'>
                {category.total_tasks_done}/{category.total_tasks}
            </span>
        </div>
    )
}

