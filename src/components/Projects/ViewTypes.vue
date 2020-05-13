<template>    
  <div class="viewTypes">
    <span 
      v-for="_viewType in viewTypes"
      :key="_viewType.id"
      class="viewTypes__item"
      :class="{'viewTypes__item--active': _viewType.id === viewType}"
      @click="$emit('view-type-selected', _viewType.id)"
    >
      <font-awesome-icon
        class="fa-3x"
        :icon="['fa', _viewType.icon]"
      />
    </span>
  </div>
</template>

<script lang="ts">

  export default {
    name: 'ViewTypes',
    props: {
        viewType: String
    },
    data() {
        return {
            viewTypes: [
                {
                    id: 'grid',
                    icon: 'th'
                },
                {
                    id: 'list',
                    icon: 'bars'
                }
            ]
        }
    }
   }
</script>

<style lang="scss" scoped>
    @import '../../utils';

    .viewTypes {
        flex: 1 1 100%;
        display: flex;
        justify-content: flex-end;
        align-items: flex-start;
        margin: 0 100px 100px 100px;

        @include onTablet {
            display: none;
        }

        &__item {
            color: white;
            cursor: pointer;
            margin: 10px;
            padding: 10px;

            &--active {
                border: 1px solid white;
            }
        }
    }

</style>