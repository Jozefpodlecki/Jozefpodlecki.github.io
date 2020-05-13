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

      const randomBorder = () => {
        const random = () => Math.floor(Math.random() * 200 + 100);
        const topLeftX = random()
        const topLeffY = random()
        const topRightX = random()
        const topRightY = random()
        const bottomRightX = random()
        const bottomRightY = random()
        const bottomLeftX = random()
        const bottomLeftY = random()
        return `border-radius: ${topLeftX}px ${topRightX}px ${bottomRightX}px ${bottomLeftX}px / ${topLeffY}px ${topRightY}px ${bottomRightY}px ${bottomLeftY}px`
      }

      const addAnimation = () => {
        const style = document.createElement('style');
        style.type = 'text/css';
        let animation = '';

        for(let it of Array(100).fill(0).map((pr, index) => index)) {
          const property = randomBorder();
          animation += `${it}% {${property}}`;
        }

        let computedStyle = `
        .randomBorder {
          animation: randomBorder 100s infinite forwards;
        }
        @keyframes randomBorder {
          ${animation}
        }
        @-webkit-keyframes randomBorder {
          ${animation}
        }
        @-moz-keyframes randomBorder {
          ${animation}
        }`;
        style.innerHTML = computedStyle;
        document.getElementsByTagName('head')[0].appendChild(style);
      }

      addAnimation();
      // this.randomBorder = randomBorder()

      // const loop = () => {
      //   this.randomBorder = randomBorder()

      //   setTimeout(loop, 1000)
      // }

      // setTimeout(loop, 1000)
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