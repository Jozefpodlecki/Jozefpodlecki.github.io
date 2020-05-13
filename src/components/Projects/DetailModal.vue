<template>
  <div
    :style="{ 'top': top }"
    :class="{'project-detail--enabled': enabled}"
    class="project-detail"
  >
    <div class="project-detail__container">
      <div class="project-detail__header">
        <div class="project-detail__title">
          {{ name }}
        </div>
        <div
          class="project-detail__close"
          @click="$emit('close')"
        >
          <font-awesome-icon :icon="['fa', 'times']" />
        </div>
      </div>
      <div class="project-detail__videoWrapper">
        <video
          :key="preview"
          class="project-detail__video"
          controls
        >
          <source
            :src="preview"
            type="video/webm"
          >
        </video>
      </div>
      <div class="project-detail__description">
        {{ description }}
      </div>
      <div class="project-detail__skills">
        <div class="project-detail__skillsHeader">
          Techstack
        </div>
        <div class="project-detail__skillContainer">
          <img
            v-for="skill in computedSkills"
            :key="skill.id"
            class="project-detail__skill"
            :src="skill.image"
          >
        </div>
      </div>
      <div class="project-detail__actions">
        <actions
          :show-info="false"
          :actions="actions"
          @info="$emit('project-info', id)"
        />
      </div>
    </div>
  </div>
</template>

<script>
  import Actions from './Actions/index.vue'
  import { skillsMap } from '../../constants'

  export default {
    name: 'Detail',
    components: {
        Actions
    },
    props: {
        top: {
            type: String,
            default: ''
        },
        enabled: {
            type: Boolean,
            default: true
        },
        name: {
            type: String,
            default: ''
        },
        description: {
            type: String,
            default: ''
        },
        preview: {
            type: String,
            default: ''
        },
        skills: {
            type: Array,
            default: () => []
        },
        actions: {
            type: Array,
            default: () => []
        }
    },
    data() {
      return {

      }
    },
    computed: {
        computedSkills: function() {
            if(!this.skills) {
                return null;
            }

            return this.skills.map(skill => ({...skillsMap[skill], ...skill}));
        }
    },
    mounted() {
      
    },
    methods: {
    }
   }
</script>

<style lang="scss">

    .project-detail__actions .infoAction,
        .project-detail__actions .iconAction
    {
        color: black;
    }

</style>

<style lang="scss" scoped>
    @import '../../utils';

    .project-detail {
        display: none;
        position: absolute;
        left: 0;
        width: 100vw;
        height: 100vh;
        justify-content: center;
        align-items: center;
        color: white;
        z-index: 1;
        background: rgba(0,0,0,0.9);

        &--enabled {
            display: flex;
        }

        &__header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 10px;
        }

        &__videoWrapper {
            padding: 0 50px;
            text-align: center
        }

        &__video {
            width: 100%;
            height: 600px;
            object-fit: cover;

            @include onMediumDesktop {
              width: 800px;
              height: 500px;
            }

            @include onDesktop {
              width: 600px;
              height: 400px;
            }

            @include onTablet {
              width: 300px;
              height: 300px;
            }
        }

        &__title {
            flex: 1 1 auto;
            color: black;
            font-size: 3rem;
            text-align: center;
        }

        &__close {
            font-size: 3rem;
            cursor: pointer;
            color: black;
            margin: 5px;
        }

        &__skills {
            flex: 1 1 auto;
            display: flex;
            flex-direction: column;
        }

        &__skillsHeader {
            font-size: 3rem;
            color: black;
            text-align: center;
            margin: 2rem 0;
        }

        &__skillContainer {
            display: flex;
            justify-content: center;
        }

        &__skill {
            display: block;
            width: 50px;
            height: 50px;
            object-fit: contain;
            margin: 0 10px;
        }

        &__imageIcon {
            width: 40px;
            height: auto;
        }

        &__actions {
            flex: 0 0 auto;
            padding: 10px;
            margin: 10px auto 10px auto;
        }

        &__action {
            color: black;
            font-size: 3rem;
            margin: 0 10px;
        }

        &__description {
            flex: 1 1 auto;
        }

        &__container {
            background: white;
            border-radius: 10px;
            display: flex;
            flex-direction: column;
            justify-content: stretch;
            align-items: stretch;
            flex: 0 1 1000px;

            @include onMediumDesktop {
              flex: 0 1 800px;
            }

            @include onDesktop {
              flex: 0 1 800px;
            }

            @include onTablet {
              flex: 0 1 600px;
            }
        }
    }

</style>