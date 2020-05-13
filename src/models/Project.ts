import { ProjectAction } from "./ProjectAction";

export interface Project {
    id: number;
    active: boolean;
    name: string;
    description: string;
    thumbnail: string;
    preview: string;
    skills: string[];
    actions: ProjectAction[];
    rank: number;
}