import classNames from 'classnames';
import { CategoriesData } from '../types';
import { NewCategoryInput } from './NewCategoryInput';
import { Category } from './Category';
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

    const allCategoryStyles = classNames({
        'category-tasks-all-done':
            categoriesData.total_tasks_done == categoriesData.total_tasks &&
            categoriesData.total_tasks != 0,
        category: true,
        'current-category': currentCategory == null,
    });

    return (
        <div className='side-bar'>
            <Category
                label='All'
                setCategoryTo={null}
                styles={allCategoryStyles}
                currentCategory={currentCategory}
                setCurrentCategory={setCurrentCategory}
                category={categoriesData}
                getTasks={getTasks}
                getCategories={getCategories}
            />
            <NewCategoryInput
                setCurrentCategory={setCurrentCategory}
                getCategories={getCategories}
            />
            {categoriesData.categories.map((x) => {
                const styles = classNames({
                    'category-tasks-all-done':
                        x.total_tasks_done == x.total_tasks &&
                        x.total_tasks != 0,
                    category: true,
                    'current-category': currentCategory == x.name,
                });

                return (
                    <Category
                        key={x.name}
                        label={x.name}
                        setCategoryTo={x.name}
                        styles={styles}
                        setCurrentCategory={setCurrentCategory}
                        getCategories={getCategories}
                        currentCategory={currentCategory}
                        getTasks={getTasks}
                        category={x}
                    />
                );
            })}
        </div>
    );
}
