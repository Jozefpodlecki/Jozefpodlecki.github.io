<template>
  <canvas
    id="canvas"
    ref="canvas"
    :width="width"
    :height="height"
  />
</template>

<script lang="ts">

  export default {
    name: 'BackgroundCanvas',
    data() {
      return {
        width: 0,
        height: 0,
        background: '#0c091d',
        particles: [] as any[]
      }
    }
   }
</script>

<style lang="scss" scoped>

</style>