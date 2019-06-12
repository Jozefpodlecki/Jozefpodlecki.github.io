export type Profile = {
    summary?: string,
    address?: string,
    titles?: string[],
    fullName?: string,
    avatarUrl?: string,
    startTitle?: string
}

export interface IState {
    profile: Profile
}