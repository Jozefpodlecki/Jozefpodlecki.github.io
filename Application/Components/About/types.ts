export type Skill = {
    name: string,
    level: number
}

export type Career = {
    range: string,
    title: string,
    company: string,
    city: string,
    country: string,
    responsibilities: string[]
}

export type Profile = {
    summary?: string,
    address?: string
}

export interface IState {
    skills: Skill[],
    education: any[],
    career: Career[],
    profile: Profile
}