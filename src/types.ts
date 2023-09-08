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

export interface T_TaskInRes {
    id: string;
    category_name: string;
    title: string;
    desc?: string;
    status: number;
    priority: number;
    created_at: number;
    done_at?: number;
    updated_at: number;
    sub_tasks: T_TaskInRes[],
}

