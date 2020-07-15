<template>
  <div class="contact">
    <div class="contact__header">
      Get in touch!
    </div>
    <div class="contact__container">
      <div class="contact__form">
        <form @submit.prevent="contactSubmit">
          <div class="contact__field">
            <input
              v-model="form.email"
              required
              class="contact__input"
              type="text"
              placeholder="Email"
            >
          </div>
          <div class="contact__field">
            <input
              v-model="form.title"
              required
              class="contact__input"
              type="text"
              placeholder="Title"
            >
          </div>
          <div class="contact__field">
            <textarea
              v-model="form.comment"
              required
              class="contact__textarea"
              placeholder="Comment"
            />
          </div>
          <div class="contact__field">
            <input
              class="contact__submit"
              type="submit"
              value="Submit"
            >
          </div>
        </form>
      </div>
      <div class="contact__mapImageWrapper">
        <img
          class="contact__mapImage"
          :src="map_image_href"
          alt="place"
        >
      </div>
    </div>
    <div class="contact__social">
      <div class="contact__socialHeader">
        You can also find me on following platforms!
      </div>
      <div class="contact__socialContainer">
           <a
          class="contact__socialIcon"
          href="https://stackoverflow.com/users/2304474/j%c3%b3zef-podlecki?tab=profile"
        >
          <font-awesome-icon :icon="['fab', 'stack-overflow']" />
        </a>
        <a
          class="contact__socialIcon"
          href="https://www.facebook.com/jozek.podlecki"
        >
          <font-awesome-icon :icon="['fab', 'facebook']" />
        </a>
        <a
          class="contact__socialIcon"
          href="https://www.freecodecamp.org/jozefpodlecki"
        >
          <font-awesome-icon :icon="['fab', 'free-code-camp']" />
        </a>
        <a
          class="contact__socialImage"
          href=""
        >
          <img
            class="contact__image"
            src="/assets/social/duolingo.jpg"
            alt="duolingo"
          >
        </a>
        <a
          class="contact__socialImage"
          href=""
        >
          <img
            class="contact__image"
            src="/assets/social/codewars.png"
            alt="codewars"
          >
        </a>
        <a
          class="contact__socialImage"
          href="https://www.anime-planet.com/users/Jozefpodlecki"
        >
          <img
            class="contact__image"
            src="/assets/social/anime_planet.jpg"
            alt="anime_planet"
          >
        </a>
        <a
          class="contact__socialImage"
          href=""
        >
          <img
            class="contact__image"
            src="/assets/social/tableau.jpg"
            alt="tableau"
          >
        </a>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import { getMap, getProfile } from '../api';
  
  export default {
    name: 'Contact',
    data() {
      return {
        form: {
          email: null as string,
          title: null as string,
          comment: null as string
        },
        map_image_href: null as string
      }
    },
    mounted () {
      this.map_image_href = getMap();

      getProfile()
        .then(profile => {

        })
    },
    methods: {
      contactSubmit() {

      }
    }
   }
</script>

<style lang="scss" scoped>
  @import '../utils';
  @import '../variables';

  .contact {
    width: 100vw;
    text-align: center;
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;

    @include onPhone {
      padding: 0 10px;
    }

    &__social {
      display: flex;
      flex-direction: column;
      justify-content: center;
      flex-wrap: wrap;

      @include onPhone {
        margin-top: 2rem;
        padding: 0 2rem;
      }
    }

    &__socialHeader {
      color: $gray;
      font-size: 8rem;
      margin-bottom: 6rem;

      @include onTablet {
          font-size: 5rem;
      }

      @include onPhone {
          font-size: 4rem;
      }
    }

    &__socialContainer {
      display: flex;
      justify-content: center;
      flex-wrap: wrap;
    }

    &__socialIcon, &__image {
      filter: grayscale(100%);
      transition: transform .3s ease-in;

      &:nth-child(odd):hover {
        transform: rotate(45deg);
      }

      &:nth-child(even):hover {
        transform: rotate(-45deg);
      }
    }

    &__socialImage, &__socialIcon {
      margin: 0 20px 20px 20px;
    }

    &__socialImage {
      display: block;
    }

    &__image {
      width: 100px;
      height: 100px;
      border-radius: 20px;

      @include onPhone {
        width: 50px;
        height: 50px;
      }
    }

    &__socialIcon {
      color: $white;
      font-size: 9rem;

       @include onPhone {
        font-size: 5rem;
      }
    }

    &__header {
      flex: 1 1 100%;
      color: $gray;
      font-size: 8rem;
      margin-bottom: 6rem;
      flex: 0 0 100%;

      @include onPhone {
        font-size: 4rem;
      }
    }

    &__container {
      display: flex;
      justify-content: center;
      align-items: center;
      flex-wrap: wrap;
      flex: 1 1 auto;
      padding: 0 7rem 0 2rem;
    }

    &__form {
      flex: 0 1 500px;
      margin: 0 50px 0 50px;
      @include onPhone {
        font-size: 4rem;
      }
    }

    &__mapImageWrapper {
      flex: 0 1 600px;
      padding: 2rem 5rem;

      @include onTablet {
        margin-top: 2rem;
        padding: 2rem 10rem;
      }
    }

    &__mapImage {
      border-radius: 10px;
      width: 100%;
    }

    &__submit {
      font-size: 2rem;
      color: $gray;
      padding: 10px 20px;
      border-radius: 10px;
      border: none;
      transition: box-shadow .3s ease-in;
      box-shadow: 0px 1px 10px 0px $white;
      cursor: pointer;

      &:hover {
        box-shadow: 0px 1px 10px 1px $white;
      }
    }

    &__field {
      margin-top: 20px;
    }

    &__textarea, &__input {
      font-size: 2rem;
      padding: 10px;
      width: 100%;
      border-radius: 10px;
      border: none;
      background: #0c091d;
      color: white;
      transition: box-shadow .3s ease-in;
      box-shadow: 0px 1px 10px 0px $white;

      &:hover {
        box-shadow: 0px 1px 10px 1px $white;
      }

      &:focus {
        outline: none !important;
        box-shadow: 0px 1px 10px 2px $white;
      }
    }

    &__textarea {
      height: 100px;
      resize: vertical;
    }
  }

</style>