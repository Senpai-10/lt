import classNames from 'classnames';
import { invoke } from '@tauri-apps/api';
import { T_Task } from '../types';
import '../css/components/Task.css';

import CheckboxCheckedIcon from '../assets/checked_checkbox.svg';
import CheckboxUncheckedIcon from '../assets/unchecked_checkbox.svg';
import ShowMore from '../assets/showmore.svg';
import TrashIcon from '../assets/trash.svg';

interface Props {
    task: T_Task;
    getCategories: () => void;
    getTasks: () => void;
}

export function Task(props: Props) {
    const { task, getCategories, getTasks } = props;

    const toggleStatus = () => {
        const newStatus = task.status == 0 ? 1 : 0;
        invoke('update_task_status', { id: task.id, status: newStatus }).then(
            () => {
                getTasks();
                getCategories();
            },
        );
    };

    const handleRemoveTask = () => {
        invoke('remove_task', { id: task.id }).then(() => {
            getTasks();
            getCategories();
        })
    }

    return (
        <div className='task'>
            <div className='task-status' onClick={toggleStatus}>
                {task.status != 0 ? (
                    <img src={CheckboxCheckedIcon} />
                ) : (
                    <img src={CheckboxUncheckedIcon} />
                )}
            </div>
            <div className='task-title-desc'>
                <span
                    className={classNames({
                        'task-title': true,
                        'done-task-title': task.status != 0,
                    })}
                    title={task.title}
                >
                    {task.title}
                </span>
                <span
                    title={task.desc}
                    className={classNames({
                        'task-desc': true,
                        'done-task-desc': task.status != 0,
                    })}
                >
                    {task.desc || 'desc..'}
                </span>
            </div>
            <div className='task-extra-icon'>
                <img src={ShowMore} />
                <img className="remove-icon" onClick={handleRemoveTask} src={TrashIcon} />
            </div>
        </div>
    );
}
