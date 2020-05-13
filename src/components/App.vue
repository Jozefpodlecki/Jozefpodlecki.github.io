<template>
  <div>
    <top-menu 
      :menu-items="menuItems"
      :author="author"
      :visible="topMenuAuthorVisible"
      @click="goToSection"
    />
    <music />
    <home
      :author="author"
      :title="title"
      :github-link="githubLink"
      :linked-in-link="linkedInLink"
      :email="email"
      @scroll-down="scrollDown"
    />
    <about
      :avatar="avatar"
      :description="description"
    />
    <projects
      :view-type="viewType"
      :page-size="pageSize"
      @view-type-selected="viewTypeSelected"
    />
    <contact />
    <site-footer
      :author="author"
      :social="social"
    />
    <scrollbar
      :menu-items="menuItems"
      @click="goToSection"
    />
  </div>
</template>

<script lang="ts">
  import { getProfile, getMenuItems } from '../api'

  import TopMenu from './TopMenu.vue'
  import Music from './Music.vue'
  import Home from './Home.vue'
  import About from './About.vue'
  import Projects from './Projects/index.vue'
  import Contact from './Contact.vue'
  import Scrollbar from './Scrollbar.vue'
  import SiteFooter from './Footer.vue'

  export default {
    name: 'App',
    components: {
      Music,
      TopMenu,
      Home,
      About,
      Projects,
      Contact,
      Scrollbar,
      SiteFooter
    },
    data() {
      return {
        menuItems: [] as any[],
        social: [] as any[],
        avatar: null as string,
        author: null as string,
        title: null as string,
        description: null as string,
        githubLink: null as string,
        linkedInLink: null as string,
        email: null as string,
        viewType: 'list',
        topMenuAuthorVisible: false,
        pageSize: -1
      }
    },
    created () {
      window.addEventListener('scroll', this.handleScroll);
      window.addEventListener('resize', this.handleResize);
    },
    destroyed () {
      window.removeEventListener('scroll', this.handleScroll);
      window.removeEventListener('resize', this.handleResize);
    },
    mounted() {
      this.handleResize()
      getProfile().then(({avatar, author, title, description, githubLink, linkedInLink, email, social}) => {
        this.avatar = avatar;
        this.author = author;
        this.description = description;
        this.title = title;
        this.githubLink = githubLink;
        this.linkedInLink = linkedInLink;
        this.email = email;
        this.social = social;
      })

      getMenuItems().then(menuItems => {
          
          setTimeout(() => {

            const baseHeight = document.documentElement.scrollTop;

            const getTop = (className) => document.getElementsByClassName(className)[0].getBoundingClientRect().top;

            const menuItemsHeight = [
              0,
              baseHeight + getTop("about") - 100,
              baseHeight + getTop("projects") - 100,
              baseHeight + getTop("contact") - 100
            ]

            this.menuItems = menuItems.map((menuItem, index) => ({...menuItem, height: menuItemsHeight[index], selected: false}));
            this.handleScroll()
          }, 500)
      })
    },
    methods: {
      scrollDown() {
        let menuItem = this.menuItems[1];
        window.scrollTo({
          top: menuItem.height,
          behavior: 'smooth'
        })
      },
      viewTypeSelected(viewType) {
        this.viewType = viewType;
      },
      handleResize() {
        const width = window.innerWidth;
        this.pageSize = 8;
        
        if(width < 1700) {
          this.pageSize = 8;
        }
        if(width < 1400) {
          this.pageSize = 4;
        }
        if(width < 700) {
          this.viewType = 'list'
        }
      },
      handleScroll () {
        
        if(!this.menuItems.length) {
          return
        }

        const scrollY = window.scrollY;
        let id = null;

        let lastMenuItem = this.menuItems[0]
        for(let menuItem of this.menuItems.slice(1)) {
          if(scrollY >= lastMenuItem.height && scrollY < menuItem.height) {
            id = lastMenuItem.id;
            break;
          }
          lastMenuItem = menuItem;
        }

        if(!id) {
          id = this.menuItems[this.menuItems.length - 1].id;
        }

        if(id !== 1) {
          this.topMenuAuthorVisible = true;
        }
        else {
          this.topMenuAuthorVisible = false;
        }

        let menuItem = this.menuItems.find(menuItem => menuItem.selected);
        let newMenuItem = this.menuItems.find(menuItem => menuItem.id === id);
        if(menuItem) {

          if(newMenuItem.id === menuItem.id) {
            return;
          }

          this.$set(menuItem, 'selected', false)
        }
        
        this.$set(newMenuItem, 'selected', true)
      },
      goToSection(id) {
          const menuItem = this.menuItems.find(menuItem => menuItem.id === id);
          debugger;
          window.scrollTo({
              top: menuItem.height,
              behavior: 'smooth'
          })
      }
    }
  }
</script>

<style lang="scss" scoped>
</style>