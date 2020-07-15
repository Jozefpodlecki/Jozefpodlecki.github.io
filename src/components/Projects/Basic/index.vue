<template>
  <div class="projects-basic">
    <div class="projects-basic__container">
      <project-basic-item
        v-for="project in projects"
        :key="project.id"
        v-bind="project"
        class="projects-basic__item"
        @project-info="$emit('project-info', project)"
      />
    </div>
  </div>
</template>

<script lang="ts">
  import { getProjects } from '@api';
  import { Project } from '@models';
  import ProjectBasicItem from './Basic-Item.vue'

  export default {
    name: 'ProjectsBasic',
    components: {
      ProjectBasicItem
    },
    data() {
      return {
        projects: [] as Project[]
      }
    },
    mounted() {
      getProjects({
        pageSize: 6,
        page: 0
      }).then(({data, total}) => {
        this.projects = data.map(pr => ({
          ...pr,
          state: 'thumbnail'
        }));
      })
    },
    methods: {
      videoLoaded(event: EventWithTarget<HTMLAudioElement>, project: any) {
        const video = event.target;
        this.$set(project, 'state', 'video')
        video.play();
      },
      stopAction(id) {
        const video = this.$refs.video;

        if(!video) {
          return;
        }

        video.stop();
      },
      startVideo(event: EventWithTarget<HTMLVideoElement>, project: any) {
        
        if(project.state == "loading") {
          return;
        }

        if(project.state == "video") {
          const video = event.target as HTMLVideoElement;

          if(video.currentTime > 0 && !video.paused && !video.ended && video.readyState > 2) {
            return;
          }

          video.play();
          return;
        }

        this.$set(project, 'state', 'loading')

        new Promise((resolve, reject) => {
          setTimeout(resolve, 1000)
        }).then(pr => {
          
        })
      }
    }
   }
</script>

<style lang="scss" scoped>
  .projects-basic {
    width: 100vw;
    display: flex;
    flex-wrap: wrap;
    margin-bottom: 6rem;

    &__container {
      display: grid;
      grid-template-columns: repeat(3, 300px);
      grid-template-rows: repeat(auto-fit, 300px);
      grid-row-gap: 20px;
      grid-column-gap: 100px;
      width: 100%;
      height: 100%;
      justify-content: center;
      margin: 0 40px;

      @media screen and (max-width: 1200px) {
        grid-template-columns: repeat(auto-fit, 300px);
      }
    }

    &__item {
        overflow: hidden;
        border-radius: 10px;
        display: flex;
        flex-direction: column;
    }
  }
</style>