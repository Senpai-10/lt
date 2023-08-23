import classNames from 'classnames';

import '../css/components/SidebarItem.css';

interface Props {
    name: string;
    type: 'category-name' | 'all-tasks';
    total: { done: number; tasks: number };
    is_active: boolean;
    setCurrentCategory: React.Dispatch<React.SetStateAction<string | null>>;
}

export function SidebarItem(props: Props) {
    const { name, total, is_active, type, setCurrentCategory } = props;

    const handleClick = () => {
        setCurrentCategory(type == 'all-tasks' ? null : name);
    };

    return (
        <div
            onClick={handleClick}
            className={classNames({
                'sidebar-item': true,
                'sidebar-item-active': is_active,
                'done-category':
                    type == 'category-name' &&
                    total.tasks == total.done &&
                    total.tasks != 0,
                'sidebar-item-all-tasks': type == 'all-tasks',
                'sidebar-item-all-tasks-active':
                    type == 'all-tasks' && is_active,
            })}
        >
            <span className='sidebar-item-name'>{name}</span>
            <div className='sidebar-item-tasks-count'>
                {total.done}/{total.tasks}
            </div>
        </div>
    );
}
