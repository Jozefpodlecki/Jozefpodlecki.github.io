<template>
  <div>
    <div class="project-basic__header">
      <span class="project-basic__name">{{ name }}</span>
    </div>
    <div
      class="project-basic__thumbnail"
      @mouseover="startVideo"
      @mouseleave="stopAction"
    >
      <img
        v-if="state == 'thumbnail'"
        class="project-basic__image"
        :src="thumbnail"
      >
      <div
        v-if="state == 'loading'"
        class="project-basic__loader"
        :style="{
          background: `url(${thumbnail})`,
          backgroundPosition: 'center',
          backgroundSize: 'contain'
        }"
      >
        <div class="dot-flashing" />
      </div>
      <video
        v-show="state == 'video'"
        ref="video"
        class="project-basic__video"
        muted
        @loadeddata="videoLoaded"
      >
        <source
          v-if="['loading','video'].includes(state)"
          :src="preview"
          type="video/webm"
        >
      </video>
    </div>
    <div class="project-basic__actions">
      <actions
        :actions="actions"
        @info="$emit('project-info')"
      />
    </div>
  </div>
</template>

<script lang="ts">
  import Actions from '../Actions/index.vue'

  export default {
    name: 'ProjectBasicItem',
    components: {
      Actions
    },
    props: {
        name: String,
        thumbnail: String,
        preview: String,
        actions: Array
    },
    data() {
        return {
            state: 'thumbnail',
        }
    },
    methods: {
      videoLoaded(event: EventWithTarget<HTMLVideoElement>) {
        const video = event.target;
        this.state = 'video';
        video.play();
      },
      stopAction() {
        const video = this.$refs.video as HTMLVideoElement;

        if(!video) {
          return;
        }

        video.pause();
      },
      startVideo(event: EventWithTarget<HTMLVideoElement>) {
        
        if(this.state == "loading") {
          return;
        }

        if(this.state == "video") {
          const video = this.$refs.video as HTMLVideoElement;

          if(video.currentTime > 0 && !video.paused && !video.ended && video.readyState > 2) {
            return;
          }

          video.play();
          return;
        }

        this.state = 'loading';

        new Promise((resolve, reject) => {
          setTimeout(resolve, 1000)
        });
      }
    }
   }
</script>

<style lang="scss">
  .iconAction {
    font-size: 2rem;
  }
</style>

<style lang="scss" scoped>
 
    .project-basic {

        &__header {
            flex: 0 0 auto;
            padding: 20px 0;
            text-align: center;
            background: gray;
        }

        &__name {
            color: white;
            font-size: 25px;
        }

        &__thumbnail {
            flex: 1 1 auto;
            display: flex;
        }

        &__actions {
            flex: 0 0 auto;
            color: white;
            background: gray;
            padding: 10px;
            display: flex;
            justify-content: space-around;
        }

        &__action {
            cursor: pointer;
        }

        &__loader, &__image, &__video {
            width: 100%;
            height: 100%;
        }

        &__loader {
            display: flex;
        }

        &__image, &__video {
            object-fit: cover;
        }
    }

    .dot-flashing {
        margin: auto;
        position: relative;
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background-color: #9880ff;
        color: #9880ff;
        animation: dotFlashing 1s infinite linear alternate;
        animation-delay: .5s;
    }

    .dot-flashing::before, .dot-flashing::after {
        content: '';
        display: inline-block;
        position: absolute;
        top: 0;
    }

    .dot-flashing::before {
        left: -15px;
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background-color: #9880ff;
        color: #9880ff;
        animation: dotFlashing 1s infinite alternate;
        animation-delay: 0s;
    }

    .dot-flashing::after {
        left: 15px;
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background-color: #9880ff;
        color: #9880ff;
        animation: dotFlashing 1s infinite alternate;
        animation-delay: 1s;
    }

    @keyframes dotFlashing {
        0% {
            background-color: #9880ff;
        }
        50%,
        100% {
            background-color: #ebe6ff;
        }
    }

</style>