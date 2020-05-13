<template>
  <a
    :href="link"
    target="_blank"
    :class="{ [className]: true }"
  >
    <font-awesome-icon :icon="icon" />
  </a>
</template>

<script lang="ts">

    export default {
        name: 'IconAction',
        props: {
            link: String,
            className: String,
            icon: Array
        }
    }
</script>

<style lang="scss">

    .iconAction {
        color: white;
        margin: 0 10px;
        font-size: 3rem;
        cursor: pointer;
    }

</style>