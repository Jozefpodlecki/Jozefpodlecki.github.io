import Vue from 'vue';
import VueRouter from 'vue-router';
import { library } from '@fortawesome/fontawesome-svg-core'
import { faTimes, faFlag, faTrash, faGlobe, faFile, faUser, faVolumeUp, faVolumeMute, faInfo, faBars, faTh, faEnvelope, faCopyright } from '@fortawesome/free-solid-svg-icons'
import { faGithub, faLinkedin, faBitbucket, faGitlab, faCodepen, faFreeCodeCamp, faFacebook, faStackOverflow } from '@fortawesome/free-brands-svg-icons'
import { FontAwesomeIcon, FontAwesomeLayersText, FontAwesomeLayers } from '@fortawesome/vue-fontawesome'

import App from './components/App';
import routes from './routes';
import './style.scss'

Vue.config.productionTip = false

library.add(faBars);
library.add(faTh);
library.add(faVolumeUp);
library.add(faVolumeMute);
library.add(faFile);
library.add(faCopyright);
library.add(faUser);
library.add(faTimes);
library.add(faInfo);
library.add(faFlag);
library.add(faTrash);
library.add(faEnvelope);
library.add(faGlobe);
library.add(faStackOverflow);
library.add(faFreeCodeCamp);
library.add(faFacebook);
library.add(faBitbucket);
library.add(faGitlab);
library.add(faCodepen);
library.add(faGithub);
library.add(faLinkedin);

Vue.component('font-awesome-layers', FontAwesomeLayers)
Vue.component('font-awesome-layers-text', FontAwesomeLayersText)
Vue.component('font-awesome-icon', FontAwesomeIcon)
Vue.use(VueRouter);

const router = new VueRouter({
    routes,
    linkActiveClass: 'active',
    mode: 'history'
});

new Vue({
    el: '#root',
    render: hack => hack(App),
    router
});