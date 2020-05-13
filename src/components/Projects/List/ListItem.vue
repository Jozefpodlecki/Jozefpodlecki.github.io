<template>
  <div class="list-item" @click="rowClick($event.target, id)">
    <div class="list-item__header">
      {{ name }}
    </div>
    <div class="list-item__actions">
      <actions
        :actions="actions"
        @info="$emit('project-info', id)"
      />
    </div>
  </div>
</template>

<script lang="ts">
    import Actions from '../Actions/index.vue'

    export default {
        name: 'ListItem',
        components: {
            Actions
        },
        props: {
            id: {
              type: Number
            },
            actions: {
              type: Array
            },
            name: {
              type: String
            }
        },
        methods: {
          rowClick(element: HTMLElement, id: number) {
            if(element.classList.contains("list-item") || element.classList.contains("list-item__header")) {
              this.$emit('project-info', id)
            }
          }
        }
    }
</script>

<style lang="scss" scoped>
@import '../../../utils';
@import '../../../variables';

.list-item {
    display: flex;
    padding: 1rem;
    cursor: pointer;
    transition: background .3s ease-in;

    &:hover {
      background: transparentize($white, 0.8);
    }

    &__header {
        color: $white;
        font-size: 2rem;

        @include onPhone {
            font-size: 2rem;
        }
    }

    &__actions {
      margin-left: auto;
    }

  }

</style>