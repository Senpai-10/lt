import { invoke } from '@tauri-apps/api';
import { T_Category, CategoriesData } from "../types"
import "../css/components/Category.css"

interface Props {
    label: string
    category: T_Category | CategoriesData
    styles: string
    setCategoryTo: string | null,
    currentCategory: string | null
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    getCategories: () => void;
    getTasks: () => void;
}

export function Category(props: Props) {
    const { label, category, styles, setCategoryTo, currentCategory, setCurrentCategory, getCategories, getTasks } = props;

    const handleOnDrop = (e: React.DragEvent, category_name: string | null) => {
        if (category_name == currentCategory || currentCategory == null || category_name == null) return;

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
            className={styles}
            onClick={() => setCurrentCategory(setCategoryTo)}
            onDrop={(e) => handleOnDrop(e, setCategoryTo)}
            onDragOver={(e) => e.preventDefault()}
        >
            <span>{label}</span>
            <span className='category-tasks-count'>
                {category.total_tasks_done}/{category.total_tasks}
            </span>
        </div>
    )
}

