<template>
  <transition-group
    name="grid-item"
    class="grid"
    tag="div"
  >
    <grid-item
      v-for="project in projects"
      :key="project.id"
      v-bind="project"
      @project-info="$emit('project-info', project)"
    />
  </transition-group>
</template>

<script lang="ts">
  import GridItem from './GridItem.vue'

  export default {
    name: 'ProjectsGrid',
    components: {
      GridItem
    },
    props: {
      projects: Array
    }
   }
</script>

<style lang="scss" scoped>

  .grid {
      flex: 1 1 auto;
      display: grid;
      grid-template-columns: repeat(auto-fit, 300px);
      grid-template-rows: repeat(auto-fit, 300px);
      grid-gap: 10px;
      padding-right: 20px;
  }

  .grid-item-enter-active, .grid-item-leave-active {
    transition: all 1s;
  }

  .grid-item-leave-active {
    display: none;
  }

  .grid-item-enter, .grid-item-leave-to {
    opacity: 0;
  }

</style>