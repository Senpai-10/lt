import { invoke } from "@tauri-apps/api";

interface Props {
    currentCategory: string | null;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    tasksSearchQuery: string
    setTasksSearchQuery: React.Dispatch<React.SetStateAction<string>>
    hideDone: boolean
    setHideDone: React.Dispatch<React.SetStateAction<boolean>>
    getCategories: () => void;
}

export function Navbar(props: Props) {
    const {
        currentCategory,
        setCurrentCategory,
        tasksSearchQuery,
        setTasksSearchQuery,
        hideDone,
        setHideDone,
        getCategories,
    } = props;

    const removeCategory = () => {
        if (currentCategory == null) return;

        invoke('remove_category', { name: currentCategory });
        setCurrentCategory(null);
        getCategories();
    };

    return (
        <div className='task-list-nav'>
            <div className='filtering-settings'>
                <input
                    placeholder='search'
                    value={tasksSearchQuery}
                    onChange={(e) => setTasksSearchQuery(e.currentTarget.value)}
                />
                <label>
                    Hide Done
                    <input
                        type='checkbox'
                        checked={hideDone}
                        onChange={() => setHideDone(!hideDone)}
                    />
                </label>
            </div>
            {currentCategory != null ? (
                <button className='remove-btn' onClick={removeCategory}>
                    Del {currentCategory}
                </button>
            ) : null}
        </div>
    );
}
