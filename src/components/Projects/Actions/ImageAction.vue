<template>
  <a
    :href="link"
    target="_blank"
  >
    <img
      :class="{ [className]: true }"
      :src="src"
    >
  </a>
</template>

<script lang="ts">

    export default {
        name: 'ImageAction',
        props: {
            link: String,
            className: String,
            src: String
        }
    }
</script>

<style lang="scss">

    .imageAction {
        width: 30px;
        height: auto;
    }

</style>