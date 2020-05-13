<template>
  <div class="pagination">
    <ul class="pagination__list">
      <li
        :class="{'pagination__item--disabled': currentPage === 0 || [0, 1].includes(pages.length)}"
        class="pagination__item"
        @click="goToPage(currentPage - 1)"
      >
        &lt;
      </li>
      <template v-if="pages.length > 1">
        <li
          v-for="item in pages"
          :key="item"
          :class="{'pagination__item--selected': currentPage === item - 1}"
          class="pagination__item"
          @click="goToPage(item - 1)"
        >
          {{ item }}
        </li>
      </template>
      <li
        :class="{'pagination__item--disabled': currentPage === pages.length - 1 || [0, 1].includes(pages.length)}"
        class="pagination__item"
        @click="goToPage(currentPage + 1)"
      >
        &gt;
      </li>
    </ul>
  </div>
</template>

<script lang="ts">

  export default {
    name: 'Pagination',
    props: {
        currentPage: Number,
        numberOfPages: Number
    },
    computed: {
        pages: function() {
            if(!this.numberOfPages) {
                return [];
            }

            return Array(this.numberOfPages).fill(0).map((pr, index) => index + 1);
        }
    },
    methods: {
        goToPage(page: number = 0) {
            this.$emit('page-selected', page)
        }
    }
   }
</script>

<style lang="scss" scoped>
    @import '../utils';

    .pagination {
        flex: 0 0 auto;
        color: white;
        padding: 20px 0;

        @include onTablet {
            display: none;
        }

        &__list {
            display: flex;
            list-style: none;
            margin: 0;
            padding: 0;
            justify-content: center;
        }

        &__item {
            font-size: 3rem;
            font-weight: bold;
            margin: 0 5px;
            cursor: pointer;
            user-select: none;

            &--disabled {
                visibility: hidden;
            }

            &--selected {
                border: 1px solid white;  
            }
        }
    }

</style>