<template>
  <div class="projects-detailed">
    <detail-modal
      :top="detailModalTop"
      v-bind="project"
      :enabled="detailModalOpened"
      @close="detailModalOpened = false"
    />
    <view-types
      :view-type="viewType"
      @view-type-selected="$emit('view-type-selected', $event)"
    />
    <project-filter
      :tags="tags"
      @tagSelected="tagSelected"
      @search="onSearch"
    />
    <div class="projects-detailed__container">
      <projects-list
        v-if="viewType == 'list'"
        :projects="projects"
        @project-info="openModal"
      />
      <projects-grid
        v-else
        :projects="projects"
        @project-info="openModal"
      />
      <pagination
        :number-of-pages="numberOfPages"
        :current-page="currentPage"
        @page-selected="pageSelected"
      />
    </div>
  </div>
</template>

<script lang="ts">
  import { getTags, getProjects } from '@api'
  import DetailModal from './DetailModal.vue'
  import ViewTypes from './ViewTypes.vue'
  import Pagination from '../Pagination.vue'
  import ProjectsList from './List/index.vue'
  import ProjectsGrid from './Grid/index.vue'
  import ProjectFilter from './Filter.vue'
import { ProjectSearchCriteria } from '@models'

  export default {
    name: 'ProjectsDetailed',
    components: {
      DetailModal,
      ViewTypes,
      Pagination,
      ProjectsList,
      ProjectsGrid,
      ProjectFilter
    },
    props: {
      viewType: String,
      pageSize: Number,
    },
    data() {
      return {
        detailModalTop: "0",
        detailModalOpened: false,
        project: null,
        numberOfPages: 0,
        currentPage: 1,
        projects: [],
        tags: [],
        name: null
      }
    },
    watch: {
      pageSize: function() {
        this.onSearch({
          active: true,
          page: this.currentPage,
          name: this.name
        })
      },
      currentPage: function() {
        this.onSearch({
          active: true,
          page: this.currentPage,
          name: this.name
        })
      },
      tags: {
        handler: function() {
          this.onSearch({
            active: true,
            page: 0,
            name: this.name
          })
        },
        deep: true
      }
    },
    mounted() {
      getTags().then((tags: any) => {
        this.tags = tags.map(pr => ({
          ...pr,
          selected: false
        }));
      })
    },
    methods: {
      tagSelected(tag: any) {
        this.$set(tag, 'selected', !tag.selected)
      },
      onSearch(criteria: ProjectSearchCriteria) {
        if(!criteria.page) {
          criteria.page = 0;
        }

        const tags = this.tags.filter(pr => pr.selected).map(pr => pr.name);
        const pageSize = this.pageSize;
        this.name = criteria.name;
        const name = this.name;
        this.currentPage = criteria.page;
        const page = this.currentPage;

        criteria = {
          name,
          tags,
          page,
          pageSize,
          ...criteria
        };

        this.searchProjects(criteria)
      },
      pageSelected(page: any) {
        this.currentPage = page;
      },
      openModal(project: any) {
        this.detailModalOpened = true;
        this.detailModalTop = `${window.scrollY}px`;
        this.project = project;
      },
      searchProjects(criteria: ProjectSearchCriteria) {
        getProjects(criteria).then(({data, total}) => {
          this.projects = data;
          this.numberOfPages = total;
        })  
      }
    }
   }
</script>

<style lang="scss" scoped>
  @import '../../utils';

  .projects-detailed {
    width: 100vw;
    display: flex;
    flex-wrap: wrap;

    &__container {
      display: flex;
      flex-direction: column;
      align-items: stretch;
      flex: 1 1 auto;
      height: 730px;
      padding: 0 100px 0 0;

      @include onDesktop {
        height: auto;
        padding: 0 100px;
      }

      @include onTablet {
        
      }

      @include onPhone {
        padding: 0 10px;
      }

    }
  }

</style>