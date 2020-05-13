<template>
  <div class="home">
    <background-canvas />
    <div class="home__container">
      <div class="home__header">
        <h1 class="home__name">
          {{ author }}
        </h1>
        <h2 class="home__title">
          {{ title }}
        </h2>
        <div class="home__actions">
          <a
            class="home__socialIcon"
            :href="githubLink"
          >
            <font-awesome-icon :icon="['fab', 'github']" />
          </a>
          <a
            class="home__socialIcon"
            :href="linkedInLink"
          >
            <font-awesome-icon :icon="['fab', 'linkedin']" />
          </a>
          <a
            class="home__icon"
            :href="email"
          >
            <font-awesome-icon :icon="['fa', 'envelope']" />
          </a>
          <a
            class="home__icon"
            :href="resumeLink"
            target="_blank"
          >
            <font-awesome-layers>
              <font-awesome-icon :icon="['fa', 'file']" />
              <font-awesome-icon
                :icon="['fa', 'user']"
                :style="{color: 'black'}"
                transform="shrink-10"
              />
              <font-awesome-layers-text
                :style="{color: 'black', fontWeight: 'bold'}"
                value="CV"
                transform="down-6 shrink-13"
              />
            </font-awesome-layers>
          </a>
        </div>
        <chevron @click="$emit('scroll-down')" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import BackgroundCanvas from './BackgroundCanvas';
  import Chevron from './Chevron.vue';
  import * as html2pdf from 'html2pdf.js';

  export default {
    name: 'Home',
    components: {
      BackgroundCanvas,
      Chevron
    },
    props: {
      author: {
        type: String
      },
      title: {
        type: String
      },
      githubLink: {
        type: String
      },
      linkedInLink: {
        type: String
      },
      email: {
        type: String
      }
    },
    data() {
      return {
        resumeLink: "/assets/Jozef_Podlecki_Resume.pdf"
      }
    },
    mounted() {
      const canvas = document.getElementById('canvas') as HTMLCanvasElement;
      let width = window.innerWidth;
      let height = window.innerHeight;
      window.addEventListener("resize", () => {
        width = window.innerWidth;
        height = window.innerHeight;
        canvas.height = height;
        canvas.width = width;  
      })
      canvas.height = height;
      canvas.width = width;
      const context = canvas.getContext('2d');
      const PIx2 = 2 * Math.PI;

      const options = {
        numberOfParticles: 60
      }

      const drawParticle = function (x, y, radius, color) {
        context.beginPath();
        context.fillStyle = color;
        context.arc(x, y, radius, 0, PIx2, false);
        context.fill();
        context.closePath();
      };

      const background = "#0c091d";
      let particles = [];
      
      const clearBackground = () => {
        context.fillStyle = background;
        context.fillRect(0, 0, canvas.width, canvas.height);
      }
      
      

      const createParticles = (numberOfParticles) => {
        return Array(numberOfParticles).fill(0).map(pr => {
          const random = (from, to) => Math.floor(Math.random() * to - from);
          const x = Math.random() * 50 + width / 2 - 50;
          const y = Math.random() * 50 + height / 2 - 50;
          const radius = Math.random() * 15;
          const direction = Math.random() * 360;
          const speed = Math.random() * 5;          
          const alpha = Math.random() * 0.5;
          const color = `rgba(${random(0, 255)}, ${random(0, 255)}, ${random(0, 255)}, ${alpha})`
          const xSpeed = speed * Math.sin(direction) / Math.sin(speed);
          const angle = 90 - direction;
          const ySpeed = speed * Math.sin(angle) / Math.sin(speed);
          const outOfRange = false;

          return {
            x,
            y,
            radius,
            color,
            xSpeed,
            ySpeed
          }
        })
      }

      const updateParticle = (particle) => {
        const { xSpeed, ySpeed, radius } = particle;
        particle.x += xSpeed;
        particle.y += ySpeed;
        if(particle.radius > 0.5) {
          particle.radius -= 0.001;
        }
        particle.outOfRange = particle.x > width || particle.x < 0 || particle.y > height || particle.y < 0;
      };

      const loop = () => {
        
        clearBackground()
        for(const particle of particles) {
          const {x, y, radius, color, outOfRange} = particle;

          if(outOfRange) {
            continue;
          }

          drawParticle(x, y, radius, color)
          updateParticle(particle)
        }
        const filtered = particles.filter(pr => pr.outOfRange).length;
        particles = particles.filter(pr => !pr.outOfRange).concat(createParticles(filtered));
        
        requestAnimationFrame(loop)  
      }

      particles = createParticles(options.numberOfParticles);
      requestAnimationFrame(loop)
    },
    methods: {
    }
   }
</script>

<style lang="scss" scoped>
  @import '../utils';

  .home {
    width: 100vw;
    height: 100vh;
    background: rgb(12, 9, 29);
    color: transparentize(#FFF, 0.3);
    
    &__container {
      display: flex;
      justify-content: center;
      align-items: center;
      width: 100vw;
      height: 100vh;
      position: absolute;
    }

    &__header {
      text-align: center;
      margin-top: 10rem;
      padding-right: 7rem;

      @include onTablet {
        margin-top: 00px;
      }
    }

    &__name {
      font-size: 10rem;
      font-family: 'Titillium Web';
      margin: 0;
      user-select: none;

      @include onPhone {
        font-size: 6rem;
      }
    }

    &__title {
      font-size: 5rem;
      font-family: 'Indie Flower';
      margin: 0;
      user-select: none;

      @include onPhone {
        font-size: 3rem;
      }
    }

    &__actions {
      padding: 10px;
    }

    &__icon, &__socialIcon {
      font-size: 5rem;
      color: transparentize(#FFF, 0.2);
      margin: 20px;
      display: inline-block;
      transition: color .5s ease-in;

      &:hover {
        color: transparentize(#FFF, 0.5);
      }

      @include onPhone {
        font-size: 4rem;
      }
    }

    &__icon {
      cursor: pointer;
    }

    #canvas {
      position: absolute;
      top: 0%;
      left: 0%;
      width: 100%;
      height: 100%;
    }

  }

</style>