import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Category, Task } from "./types"
import "./App.css";



function App() {
    const [data, setData] = useState<Task[]>();
    const [categories, setCategories] = useState<Category[]>();
    const [category, setCategory] = useState<string | null>(null);
    const [newTaskInput, setNewTaskInput] = useState("");
    const [newCategoryInput, setNewCategoryInput] = useState("");

    const getTasks = () => {
        if (category != null) {
            invoke("get_tasks", { category: category }).then((res: any) => {
                setData(res)
            })
        } else {
            invoke("get_tasks").then((res: any) => {
                setData(res)
            })
        }
    }

    const getCategories = () => {
        invoke("get_categories")
            .then((data: any) => {
                setCategories(data)
            })
    }

    useEffect(() => {
        getCategories()
    }, [])

    useEffect(() => {
        getTasks()
    }, [category])

    if (data == undefined || categories == undefined) {
        return <h1>Loading</h1>
    }

    const addCategory = () => {
        invoke("add_category", { name: newCategoryInput });
        setNewCategoryInput("")
        getCategories();
    }

    const addTask = () => {
        if (newTaskInput == "") return

        invoke("add_task", { title: newTaskInput, category: category == null ? "main" : category })
        setNewTaskInput("")
        getCategories()
        getTasks()
    }

    return (
        <div className="container">
            <div className="side-bar">
                <button onClick={() => setCategory(null)}>All</button>
                {
                    categories.map((x) => (
                        <button key={x.name} onClick={() => setCategory(x.name)}>{x.name}</button>
                    ))
                }
                <div>
                    <input className="new-category-input" onChange={(e) => setNewCategoryInput(e.currentTarget.value)} value={newCategoryInput} placeholder="category.." />
                    <button onClick={addCategory} style={{ fontWeight: "bold" }}>+</button>
                </div>
            </div>
            <div className="main-content">
                {
                    data.length == 0 && category == null ? <div><p>empty list</p></div> : data.map((task) => (
                        <div key={task.id}>
                            <span>{task.title}</span>
                        </div>
                    ))
                }
                {
                    category != null ? (
                        <>
                            <input onChange={(e) => setNewTaskInput(e.currentTarget.value)} value={newTaskInput} placeholder="task.." />
                            <button onClick={addTask}>add</button>
                        </>
                    ) : null
                }
            </div>
        </div>
    );
}

export default App;
