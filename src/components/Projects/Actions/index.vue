<template>
  <div class="actions">
    <span
      v-if="showInfo"
      class="infoAction"
      @click="$emit('info')"
    >
      <font-awesome-icon :icon="['fa', 'info']" />
    </span>
    <component
      :is="action.component"
      v-for="action in computedActions"
      :key="action.type"
      v-bind="action"
    />
  </div>
</template>

<script lang="ts">
  import IconAction from './IconAction.vue'
  import ImageAction from './ImageAction.vue'
  import { actionComponentMap } from '../../../constants'

  export default {
    name: 'Actions',
    components: {
      IconAction,
      ImageAction
    },
    props: {
      actions: Array,
      showInfo: {
        type: Boolean,
        default: true
      }
    },
    data() {
      return {
        actionComponentMap
      }
    },
    computed: {
        computedActions: function() {
          if(!this.actions) {
            return []
          }

          return this.actions.map(action => ({...actionComponentMap[action.type], ...action}))
        }
    }
   }
</script>

<style lang="scss" scoped>

  .infoAction {
    color: white;
    margin: 0 10px;
    font-size: 2rem;
    cursor: pointer;
  }

  .actions {
    display: flex;
    align-items: center;
  }

</style>