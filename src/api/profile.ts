import { MenuItem, Profile } from "@models";

export const getMenuItems = () => Promise.resolve<MenuItem[]>([
    {
        id: 1,
        name: "Home"
    },
    {
        id: 2,
        name: "About"
    },
    {
        id: 3,
        name: "Projects"
    },
    {
        id: 4,
        name: "Contact"
    }
])

export const getMap = () => {
    const url = "https://api.tomtom.com/map/1/staticimage?";
    var searchParams = new URLSearchParams();
    const width = 300;
    searchParams.append("key", "GgwjdLtaLgqGwuzl2OfpqvhP7YLjTKh5")
    searchParams.append("zoom", "9")
    searchParams.append("center", "-1.083686,53.785694")
    searchParams.append("format", "jpg")
    searchParams.append("layer", "basic")
    searchParams.append("style", "main")
    searchParams.append("width", width.toString())
    searchParams.append("height", width.toString())
    searchParams.append("view", "Unified")
    searchParams.append("language", "en-gb")

    return url + searchParams.toString();
}

export const getProfile = () => new Promise<Profile>((resolve, reject) => {
    const profile = require('../data/profile.json');

    resolve(profile);
})