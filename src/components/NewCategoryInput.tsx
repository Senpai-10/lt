import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import "../css/components/NewCategoryInput.css"

interface Props {
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    getCategories: () => void;
}

export function NewCategoryInput(props: Props) {
    const {
        setCurrentCategory,
        getCategories
    } = props;
    const [newCategoryInput, setNewCategoryInput] = useState('');

    const addCategory = () => {
        if (newCategoryInput == '') return;

        invoke('add_category', { name: newCategoryInput });
        setNewCategoryInput('');
        setCurrentCategory(newCategoryInput);
        getCategories();
    };

    return (
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
    )
}
