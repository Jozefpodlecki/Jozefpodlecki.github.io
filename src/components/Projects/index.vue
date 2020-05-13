<template>
  <div class="projects">
    <div class="projects__header">
      Projects
    </div>
    <detail-modal
      :top="detailModalTop"
      v-bind="project"
      :enabled="detailModalOpened"
      @close="detailModalOpened = false"
    />
    <projects-detailed
      v-if="seeMore"
      :view-type="viewType"
      :page-size="pageSize"
      @view-type-selected="$emit('view-type-selected', $event)"
    />
    <projects-basic
      v-if="!seeMore"
      @project-info="openModal"
    />
    <div
      class="projects__seeMore"
      @click="toggle"
    >
      {{ seeMore ? 'See less' : 'See more' }}
    </div>
  </div>
</template>

<script lang="ts">
  import ProjectsDetailed from './Projects-Detailed.vue'
  import ProjectsBasic from './Basic/index.vue'
  import DetailModal from './DetailModal.vue'

  export default {
    name: 'Projects',
    components: {
        ProjectsDetailed,
        ProjectsBasic,
        DetailModal
    },
    props: {
      viewType: String,
      pageSize: Number
    },
    data() {
        return {
            seeMore: true,
            detailModalTop: "0",
            detailModalOpened: false,
            project: null
        }
    },
    methods: {
        toggle() {
            this.seeMore = !this.seeMore;
        },
        openModal(project) {
            this.detailModalOpened = true;
            this.detailModalTop = `${window.scrollY}px`;
            this.project = project;
        },
    }
  }
</script>

<style lang="scss" scoped>
    @import '../../utils';

    .projects {
        margin-bottom: 6rem;

        &__header {
            text-align: center;
            flex: 1 1 100%;
            font-size: 8rem;
            color: gray;
            margin-bottom: 6rem;

            @include onPhone {
                font-size: 4rem;
            }
        }

        &__seeMore {
            text-align: right;
            flex: 0 0 100%;
            font-size: 5rem;
            padding-top: 50px;
            padding-right: 100px;
            color: gray;
            cursor: pointer;
            
            @include onPhone {
                display: none;
            }
        }
    }

</style>