export const skillsMap = [
    {
        image: '/assets/html.svg',
        id: 'HTML'
    },
    {
        image: '/assets/sass.png',
        id: 'Sass'
    },
    {
        image: '/assets/stylus.png',
        id: 'Stylus'
    },
    {
        image: '/assets/less.png',
        id: 'Less'
    },
    {
        image: '/assets/python.png',
        id: 'Python'
    },
    {
        image: '/assets/flask.png',
        id: 'Flask'
    },
    {
        image: '/assets/react.png',
        id: 'React'
    },
    {
        image: '/assets/d3js.png',
        id: 'D3js'
    },
    {
        image: '/assets/angular.png',
        id: 'Angular'
    },
    {
        image: '/assets/vue-js.png',
        id: 'Vue'
    },
    {
        image: '/assets/wpf.webp',
        id: 'WPF'
    },
    {
        image: '/assets/c-sharp.png',
        id: 'C#'
    },
    {
        image: '/assets/express.png',
        id: 'Express'
    },
    { 
        image: '/assets/typescript.png',
        id: 'Typescript'
    },
    { 
        image: '/assets/pug.png',
        id: 'Pug'
    },
    {
        image: '/assets/mongodb.png',
        id: 'MongoDB'
    },
    {
        image: '/assets/webpack.png',
        id: 'Webpack'
    },
    {
        image: '/assets/mongoose.png',
        id: 'Mongoose'
    },
    {
        image: "/assets/knockout-js.png",
        id: "Knockout-Js"
    },
    {
        image: "/assets/indexedDB.jpg",
        id: "IndexedDB"
    }
].reduce((acc, val) => {
    acc[val.id] = val;
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
        src: '/assets/glitch.svg'
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