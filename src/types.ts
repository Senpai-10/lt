export interface Category {
    name: string;
    total_tasks: number;
}

export interface CategoriesData {
    categories: Category[];
    total_tasks: number;
}

export interface Task {
    id: string;
    category_name: string;
    title: string;
    status: number;
    priority: number;
    creation_date: number;
    completion_date?: number;
    modification_date: number;
}
