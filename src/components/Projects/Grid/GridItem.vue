<template>
  <div class="gridItem">
    <div class="gridItem__name">
      <span>{{ name }}</span>
    </div>
    <div
      class="gridItem__thumbnail"
      @mouseover="startVideo"
      @mouseleave="stopAction"
    >
      <img
        v-if="state == 'thumbnail'"
        class="gridItem__image"
        :src="thumbnail"
      >
      <div
        v-if="state == 'loading'"
        class="gridItem__loader"
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
        class="gridItem__video"
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
    <div class="gridItem__actions">
      <actions
        :actions="actions"
        @info="$emit('project-info', id)"
      />
    </div>
  </div>
</template>

<script lang="ts">
    import Actions from '../Actions/index.vue'

    export default {
        name: 'GridItem',
        components: {
            Actions
        },
        props: {
            id: Number,
            actions: Array,
            preview: String,
            thumbnail: String,
            name: String
        },
        data() {
            return {
                state: 'thumbnail'
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

<style lang="scss" scoped>

    .gridItem {
        display: flex;
        flex-direction: column;

        &__name {
            color: white;
            text-align: center;
            font-size: 2rem;
            padding: 10px 0;
            background: gray;
        }

        &__thumbnail {
            width: 100%;
            height: auto;
            flex: 1 1 auto;
        }

        &__loader, &__image, &__video {
            width: 100%;
            height: 100%;
            object-fit: fill;
        }

        &__actions {
            display: flex;
            justify-content: space-evenly;
            padding: 10px 0;
            background: gray;
        }
    }

</style>