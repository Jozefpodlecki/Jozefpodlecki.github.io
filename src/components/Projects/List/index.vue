<template>    
  <transition-group
    name="list"
    class="project-list"
    tag="div"
  >
    <list-item
      v-for="project in projects"
      :key="project.id"
      v-bind="project"
      @project-info="$emit('project-info', project)"
    />
  </transition-group>
</template>

<script lang="ts">
  import ListItem from './ListItem.vue'

  export default {
    name: 'ProjectsList',
    components: {
      ListItem
    },
    props: {
      projects: Array
    }
   }
</script>

<style lang="scss" scoped>

  .project-list {
      
    flex: 1 0 500px;
    display: flex;
    flex-direction: column;
    justify-content: stretch;
    align-items: stretch;
  }

  .list-enter-active, .list-leave-active {
    transition: all 1s;
  }

  .list-leave-active {
    display: none;
  }

  .list-enter, .list-leave-to {
    opacity: 0;
    transform: translateY(80px);
  }

</style>