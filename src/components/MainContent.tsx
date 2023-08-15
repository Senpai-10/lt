import { T_Task } from '../types';
import { Navbar } from './Navbar';
import { NewTaskInput } from './NewTaskInput';
import { Task } from './Task';
import '../css/components/MainContent.css';

interface Props {
    tasksList: T_Task[];
    currentCategory: string | null;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
    tasksSearchQuery: string;
    setTasksSearchQuery: React.Dispatch<React.SetStateAction<string>>;
    hideDone: boolean;
    setHideDone: React.Dispatch<React.SetStateAction<boolean>>;
    getCategories: () => void;
    getTasks: () => void;
}

export function MainContent(props: Props) {
    const {
        tasksList,
        currentCategory,
        setCurrentCategory,
        tasksSearchQuery,
        setTasksSearchQuery,
        hideDone,
        setHideDone,
        getCategories,
        getTasks,
    } = props;

    return (
        <div className='main-content'>
            <Navbar
                currentCategory={currentCategory}
                tasksSearchQuery={tasksSearchQuery}
                hideDone={hideDone}
                setCurrentCategory={setCurrentCategory}
                setTasksSearchQuery={setTasksSearchQuery}
                setHideDone={setHideDone}
                getCategories={getCategories}
            />
            {tasksList.length == 0 && currentCategory == null ? (
                <div>
                    <p>empty list</p>
                </div>
            ) : (
                tasksList.map((task: T_Task) => (
                    <Task
                        task={task}
                        getCategories={getCategories}
                        getTasks={getTasks}
                    />
                ))
            )}
            {currentCategory != null ? (
                <NewTaskInput
                    currentCategory={currentCategory}
                    getTasks={getTasks}
                    getCategories={getCategories}
                />
            ) : null}
        </div>
    );
}
