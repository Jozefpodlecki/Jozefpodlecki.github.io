<template>
  <div class="about">
    <div class="about__header">
      About
    </div>
    <div class="about__container">
      <div class="about__avatarWrapper">
        <img
          class="about__avatar randomBorder"
          :style="randomBorder"
          :class="imageClass"
          :src="avatar"
          alt="avatar"
          @load="imageLoad"
        >
      </div>
      <div class="about__descriptionWrapper">
        <h4 class="about__descriptionText">
          {{ description }}
        </h4>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import { addAnimation } from 'utils';

  export default {
    name: 'About',
    props: {
      avatar: {
        type: String,
        default: ''
      },
      description: {
        type: String,
        default: ''
      }
    },
    data() {
      return {
        randomBorder: null as string,
        imageClass: null as string
      }
    },
    mounted() {
      addAnimation();
    },
    methods: {
      imageLoad() {
        this.imageClass = "about__avatar--active";
      }
    }
   }
</script>

<style lang="scss" scoped>
  
  .about {
    width: 100vw;
    display: flex;
    flex-wrap: wrap;
    justify-content: stretch;
    align-items: stretch;
    flex-direction: column;
    flex: 1 1 auto;
    margin-bottom: 6rem;

    &__header {
      font-size: 8rem;
      color: gray;
      flex: 0 0 auto;
      text-align: center;
      margin-bottom: 6rem;

      @media screen and (max-width: 500px) {
        font-size: 4rem;
      }
    }

    &__container {
      display: flex;
      flex-wrap: wrap;
      justify-content: center;
      align-items: center;
      margin: 0 100px;

      @media screen and (max-width: 500px) {
        margin: 0;
      }
    }

    &__avatarWrapper {
      flex: 0 0 auto;
      margin: 0 150px;

      @media screen and (max-width: 900px) {
        margin: 0;
      }
    }

    &__avatar {
      width: 300px;
      height: auto;
      border-radius: 300px;
      transition: all 0.3s ease-out;
      opacity: 0;

      @media screen and (max-width: 500px) {
        width: 200px;
      }

      &--active {
        opacity: 1;
      }
    }

    &__descriptionWrapper {
      flex: 1 1 500px;
      display: flex;
      justify-content: center;
      align-items: center;
      color: white;
      text-align: center;

      @media screen and (max-width: 500px) {
        padding: 0 20px;
      }

    }

    &__descriptionText {
      font-family: 'Titillium Web';
      font-size: 5rem;

      @media screen and (max-width: 700px) {
        font-size: 3rem;
      }

      @media screen and (max-width: 500px) {
        font-size: 2rem;
      }
    
    }

  }
  
</style>