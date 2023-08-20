export type TasksDisplay = "all" | "active" | "done"

export interface T_Category {
    name: string;
    total_tasks_done: number;
    total_tasks: number;
}

export interface CategoriesData {
    categories: T_Category[];
    total_tasks_done: number;
    total_tasks: number;
}

export interface T_Task {
    id: string;
    category_name: string;
    title: string;
    desc?: string;
    status: number;
    priority: number;
    creation_date: number;
    completion_date?: number;
    modification_date: number;
}
