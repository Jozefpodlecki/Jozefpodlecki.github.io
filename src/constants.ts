export const skillsMap = [
    {
        image: 'html.svg',
        id: 'HTML'
    },
    {
        image: 'sass.png',
        id: 'Sass'
    },
    {
        image: 'stylus.png',
        id: 'Stylus'
    },
    {
        image: 'less.png',
        id: 'Less'
    },
    {
        image: 'python.png',
        id: 'Python'
    },
    {
        image: 'flask.png',
        id: 'Flask'
    },
    {
        image: 'react.png',
        id: 'React'
    },
    {
        image: 'd3js.png',
        id: 'D3js'
    },
    {
        image: 'angular.png',
        id: 'Angular'
    },
    {
        image: 'vue-js.png',
        id: 'Vue'
    },
    {
        image: 'wpf.webp',
        id: 'WPF'
    },
    {
        image: 'c-sharp.png',
        id: 'C#'
    },
    {
        image: 'express.png',
        id: 'Express'
    },
    { 
        image: 'typescript.png',
        id: 'Typescript'
    },
    { 
        image: 'pug.png',
        id: 'Pug'
    },
    {
        image: 'mongodb.png',
        id: 'MongoDB'
    },
    {
        image: 'webpack.png',
        id: 'Webpack'
    },
    {
        image: 'mongoose.png',
        id: 'Mongoose'
    },
    {
        image: "knockout-js.png",
        id: "Knockout-Js"
    },
    {
        image: "indexedDB.jpg",
        id: "IndexedDB"
    },
    {
        image: "rust.png",
        id: "Rust"
    },
    {
        image: "webAssemblyLogo.svg",
        id: "WebAssembly"
    }
].reduce((acc, val) => {
    acc[val.id] = {
        ...val,
        image: `assets/skills/${val.image}`
    };
    return acc;
}, {} as any)

export const actionComponentMap = [
    {
        type: 'github',
        component: 'icon-action',
        className: 'iconAction',
        icon: ['fab', 'github']
    },
    {
        type: 'gitlab',
        component: 'icon-action',
        className: 'iconAction',
        icon: ['fab', 'gitlab']
    },
    {
        type: 'bitbucket',
        component: 'icon-action',
        className: 'iconAction',
        icon: ['fab', 'bitbucket']
    },
    {
        type: 'codepen',
        component: 'icon-action',
        className: 'iconAction',
        icon: ['fab', 'codepen']
    },
    {
        type: 'glitch',
        component: 'image-action',
        className: 'imageAction',
        src: 'assets/actions/glitch.svg'
    },
    {
        type: 'live',
        component: 'icon-action',
        className: 'iconAction',
        icon: ['fa', 'globe']
    }
].reduce((acc, val) => {
    acc[val.type] = val;
    return acc;
}, {} as any)