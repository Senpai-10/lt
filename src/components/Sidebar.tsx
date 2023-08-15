import classNames from 'classnames';
import { CategoriesData } from '../types';
import '../css/components/Sidebar.css';
import { NewCategoryInput } from './NewCategoryInput';
import { Category } from './Category';

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
            <NewCategoryInput
                setCurrentCategory={setCurrentCategory}
                getCategories={getCategories}
            />
            {categoriesData.categories.map((x) => (
                <Category
                    key={x.name}
                    label={x.name}
                    setCurrentCategory={setCurrentCategory}
                    getCategories={getCategories}
                    currentCategory={currentCategory}
                    getTasks={getTasks}
                    category={x}
                />
            ))}
        </div>
    );
}
