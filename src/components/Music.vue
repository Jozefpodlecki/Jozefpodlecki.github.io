<template>
  <div
    class="music"
    @click="toggleMusic"
  >
    <span
      v-show="!playing"
      class="music__icon"
    >
      <font-awesome-icon
        class="fa-2x"
        :icon="['fas', 'volume-up']"
      />
    </span>
    <span
      v-show="playing"
      class="music__icon"
    >
      <font-awesome-icon
        class="fa-2x"
        :icon="['fas', 'volume-mute']"
      />
    </span>
    <audio
      class="music__audio"
      :src="audio_data"
    />
  </div>
</template>

<script lang="ts">
  
  export default {
    name: 'Music',
    data() {
      return {
          audio_data: null as any,
          playing: false
      }
    },
    methods: {
        toggleMusic() {
            this.playing = !this.playing;
        }
    }
  }
</script>

<style lang="scss" scoped>

    .music {
        z-index: 1;
        padding: 10px;
        position: fixed;
        top: 0%;
        left: 0%;
        color: white;
        cursor: pointer;
        user-select: none;

        &__audio {
            display: none;
        }
    }
    
</style>