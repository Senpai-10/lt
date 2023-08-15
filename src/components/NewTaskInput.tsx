import { useState } from "react";
import { invoke } from "@tauri-apps/api";

interface Props {
    currentCategory: string | null
    getCategories: () => void
    getTasks: () => void
}

export function NewTaskInput(props: Props) {
    const {
        getCategories,
        currentCategory,
        getTasks
    } = props;

    const [newTaskInput, setNewTaskInput] = useState('');
    const addTask = () => {
        if (newTaskInput == '') return;

        invoke('add_task', {
            title: newTaskInput,
            category: currentCategory == null ? 'main' : currentCategory,
        });
        setNewTaskInput('');
        getCategories();
        getTasks();
    };

    return (
        <>
            <input
                onKeyDown={(e) =>
                    e.code == 'Enter' ? addTask() : null
                }
                onChange={(e) => setNewTaskInput(e.currentTarget.value)}
                value={newTaskInput}
                placeholder='task..'
            />
            {/*<button onClick={addTask}>add</button>*/}
        </>
    )
}

