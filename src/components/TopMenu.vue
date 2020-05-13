<template>
  <div
    class="top-menu"
    :class="{ 'top-menu--active' : toggle}"
  >
    <div class="top-menu__header">
      <div
        class="top-menu__author"
        :class="{ 'top-menu__author--active' : visible}"
      >
        {{ author }}
      </div>
      <div class="top-menu__hamburger">
        <span @click="toggleMenu">
          <font-awesome-icon
            class="top-menu__icon"
            :icon="['fa', 'bars']"
          />
        </span>
      </div>
    </div>
    <div class="top-menu__listWrapper">
      <ul class="top-menu__list">
        <li
          v-for="menuItem in menuItems"
          :key="menuItem.id"
          class="top-menu__item"
          @click="onMenuClick(menuItem.id)"
        >
          {{ menuItem.name }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">

    export default {
        name: 'TopMenu',
        props: {
            author: {
                type: String
            },
            visible: {
                type: Boolean,
                default: false
            },
            menuItems: {
                type: Array
            }
        },
        data() {
            return {
                toggle: false
            }
        },
        methods: {
            toggleMenu() {
                this.toggle = !this.toggle;
            },
            onMenuClick(id: number) {
                this.toggle = !this.toggle;
                this.$emit('click', id);
            }
        }
    }
</script>

<style lang="scss" scoped>
    @import '../utils';

    .top-menu {
        display: none;
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        z-index: 2;
        padding: 1rem 2rem;
        background: #0c091d;
        transition: height .3s ease-in;
        height: 5rem;
        overflow: hidden;

        @include onPhone {
            padding: 0 2rem;
        }

        &--active {
            height: 100vh;
        }

        &__hamburger {
            padding: 1rem;
        }

        @include onPhone {
            display: flex;
            flex-direction: column;
        }

        &__author {
            transition: opacity .5s ease-in;
            opacity: 0;

            &--active {
                opacity: 1;
            }
        }

        &__header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            flex: 0 0 auto;
            color: white;
            font-size: 2rem;
            font-family: 'Bitter';
            transition: opacity .3s ease-in;
        }

        &__icon {
            color: white;
            font-size: 3rem;
            cursor: pointer;
        }

        &__listWrapper {
            flex: 1 1 auto;
        }

        &__list {
            height: 100vh;
            list-style: none;
            padding: 0;
            margin: 0;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }

        &__item {
            color: white;
            font-size: 6rem;
            margin: 2rem 0;
        }
    }

</style>