export type Link = {
    className?: string,
    src?: string,
    url?: string
}

export type Profile = {
    summary?: string,
    address?: string
}

export interface IState {
    profile: Profile
    links: Link[]
}