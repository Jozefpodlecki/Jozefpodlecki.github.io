<template>
  <footer class="footer">
    <div class="footer__info">
      <span class="footer__copy"> <font-awesome-icon :icon="['fa', 'copyright']" /></span>{{ footer }}
    </div>
    <div class="footer__social">
      <div
        v-for="socialItem in social"
        :key="socialItem.id"
      />
    </div>
  </footer>
</template>

<script lang="ts">
  
  export default {
    name: 'SiteFooter',
    props: {
      author: {
        type: String,
        default: ''
      },
      social: {
        type: Array,
        default: () => [] as any[]
      }
    },
    computed: {
      footer() {
        const year = new Date().getFullYear();
      
        return `${year} ${this.author}`
      }
    }
  }
</script>

<style lang="scss" scoped>
  @import '../utils';

  .footer {
    padding: 50px 0;
    text-align: center;

    &__info {
      color: white;
      font-family: 'Titillium Web';
      font-size: 4rem;

      @include onPhone {
        font-size: 2rem;
      }
    }

    &__copy {
      font-size: 4rem;
      padding-right: 1rem;
    }

    &__social {
      
    }
  }

</style>