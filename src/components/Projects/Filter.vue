<template>
  <div class="filter">
    <div class="filter__searchField">
      <input
        v-model="name"
        class="filter__input"
        type="text"
        placeholder="Search for project..."
      >
    </div>
    <div class="filter__tagFilter">
      <div class="filter__tagFilterHeader">
        Filter by tags
      </div>
      <div class="filter__tagFilterContainer">
        <span
          v-for="tag in tags"
          :key="tag.id"
          class="filter__searchTag"
          :class="{'filter__searchTag--active': tag.selected}"
          @click="search(tag)"
        >{{ tag.name }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">

  export default {
    name: 'ProjectFilter',
    props: {
        tags: Array
    },
    data() {
        return {
            name: null as string
        }
    },
    watch: {
      name: function() {
        this.$emit('search', {
          name: this.name
        })
      }
    },
    methods: {
        search(tag: string) {

          if(tag) {
            this.$emit('tagSelected', tag);
          }
          
          this.$emit('search', {
            name: this.name
          })
        }
    }
   }
</script>

<style lang="scss" scoped>
    @import '../../utils';

    .filter {
        display: flex;
        flex-direction: column;
        flex: 0 0 500px;
        padding: 10px 40px;

        @include onTablet {
          display: none;
        }

        &__searchField {
          padding: 0 20px;
          margin-bottom: 10px;
        }

        &__input {
          font-size: 2rem;
          padding: 10px;
          border: none;
          width: 100%;
          background: transparent;
          border-bottom: 1px solid #FFFFFFAA;
          color: white;
        }

        &__tagFilter {
            flex: 1 1 auto;
            padding: 10px;
        }

        &__tagFilterHeader {
            color: white;
            font-size: 3rem;
            text-align: center;
            margin-bottom: 10px;
        }

        &__tagFilterContainer {
            display: flex;
            justify-content: flex-start;
            align-items: flex-start;
            flex-wrap: wrap;
            font-size: 2rem;
        }

        &__searchTag {
            padding: 10px;
            margin: 10px;
            border-radius: 20px;
            border: 2px solid white;
            color: white;
            cursor: pointer;
            transition: background .3s ease-in;

            &--active {
              background: green;
            }
        }
    }

</style>