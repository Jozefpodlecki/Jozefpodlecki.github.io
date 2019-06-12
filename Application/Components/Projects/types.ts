export type Project = {
    name: string,
    description: string,
    githubLink: string,
}

export type TabNameWithProjects = {
    tabName: string,
    projects: Project[]
}

export interface IState {
    value: number
    tabNames: string[],
    projects: Project[],
    data: any[]
}