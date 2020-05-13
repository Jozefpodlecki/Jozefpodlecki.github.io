<template>
  <div class="scrollbar">
    <ul class="scrollbar__list">
      <li
        v-for="menuItem in menuItems"
        :key="menuItem.id"
        class="scrollbar__item"
        :class="{ 'scrollbar__item--selected': menuItem.selected }"
        @click="$emit('click', menuItem.id)"
      >
        {{ menuItem.name }}
      </li>
    </ul>
  </div>
</template>

<script lang="ts">

    export default {
        name: 'Scrollbar',
        props: {
           menuItems: Array
        }
    }
</script>

<style lang="scss" scoped>
    @import '../utils';

    .scrollbar {
        position: fixed;
        top: 0;
        right: 0;
        width: 5rem;
        height: 100%;

        @include onPhone {
            display: none;
        }

        &__list {
            list-style: none;
            padding: 0;
            margin: 0;
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            cursor: pointer;
        }

        &__item {
            font-size: 2rem;
            display: flex;
            justify-content: center;
            align-items: center;
            writing-mode: vertical-rl;
            text-orientation: upright;
            letter-spacing: -2px;
            font-family: 'Bitter';
            color: lightgray;
            flex: 1 1 auto;
            transition: all 0.3s ease-in;

            &--selected {
                border-radius: 10px;
                box-shadow: 1px 1px 10px 1px white;
                color: white;
            }

            &:hover {
                color: white;
            }
        }
    }

</style>