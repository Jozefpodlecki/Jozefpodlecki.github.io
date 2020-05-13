import { Social } from "./Social";

export interface Profile {
    author: string;
    avatar: string;
    address: string[];
    title: string;
    githubLink: string;
    linkedInLink: string;
    email: string;
    description: string;
    social: Social[];
}