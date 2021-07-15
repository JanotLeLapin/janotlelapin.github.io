<script lang="ts">
  import st from './assets/stone.jpg';
  import space from './assets/space.jpg';

  import { onMount } from 'svelte';

  import { scene, camera, renderer, createScene, resize as r } from './scene';

  import * as THREE from 'three';

  let el: HTMLCanvasElement;

  onMount(() => {
    createScene(el);

    const spaceTexture = new THREE.TextureLoader().load(space);
    scene.background = spaceTexture;

    const stoneTexture = new THREE.TextureLoader().load(st);
    const stone = new THREE.Mesh(
      new THREE.BoxGeometry(5, 5, 5),
      new THREE.MeshStandardMaterial({ map: stoneTexture })
    );

    const light = new THREE.PointLight(0xffffff);
    const ambient = new THREE.AmbientLight(0xffffff, 0.6);

    camera.rotation.x = -.5;
    camera.position.set(0, 5, 10);

    light.position.set(5, 5, 5);

    scene.add(stone, light, ambient);

    const animate = () => {
      requestAnimationFrame(animate);

      stone.rotation.y += 0.005;

      renderer.render(scene, camera);
    };

    const scroll = () => {
      const t = document.body.getBoundingClientRect().top;

      stone.rotation.x = t * 0.0025;
    }

    const resize = () => {
      r();

      const ta = window.innerWidth / window.innerHeight;
      const ia = 1024 / 577;
      const factor = ia / ta;
      
      spaceTexture.offset.x = factor > 1 ? (1 - 1 / factor) / 2 : 0;
      spaceTexture.repeat.x = factor > 1 ? 1 / factor : 1;
      spaceTexture.offset.y = factor > 1 ? 0 : (1 - factor) / 2;
      spaceTexture.repeat.y = factor > 1 ? 1 : factor;
    }

    animate();

    window.addEventListener('resize', resize);
    window.addEventListener('scroll', scroll);
  });
</script>

<main>
  <canvas bind:this={el}></canvas>
  <div class="content">
    <div class="container">
      <header>
        <h1>JanotLeLapin</h1>
        <h3>Bienvenue 👋</h3>
      </header>
      <section>
        <h3>📚 Mes compétences</h3>
        <p> - Java (API de Spigot)</p>
        <p> - Next.js</p>
        <p> - Discord.js</p>
        <p> - Angular</p>
        <p> - React</p>
        <p> - Svelte</p>
      </section>
      <section>
        <h3>🏆 Mon travail</h3>
        <p> - AC UHC</p>
      </section>
      <section class="wide">
        <h3>☎️ Me contacter</h3>
        <p>Vous avez des idées ? Super, je peux les développer !</p>
        <p>Contactez moi sur Discord : <a target="_blank" href="https://discord.com/users/437953881914474523">JanotLeLapin#4989</a>.</p>
        <p>Le prix dépendra de la complexité du projet.</p>
      </section>
    </div>
    <footer>
      <h1>JanotLeLapin</h1>
      <section>
        <h3>Liens</h3>
        <p><a target="_blank" href="https://discord.com/users/437953881914474523">Discord</a></p>
        <p><a target="_blank" href="https://github.com/JanotLeLapin">GitHub</a></p>
        <p><a target="_blank" href="https://www.youtube.com/channel/UC8zxDVecLTSVLpO88Z6KvcA">YouTube</a></p>
      </section>
    </footer>
  </div>
</main>

<style lang="scss">
  :root {
    --fg-1: #e6e6e6;
    --fg-2: #cccccc;
    --fg-3: #bfbfbf;
    --bg-1: #404040;
    --bg-2: #333333;
    --bg-3: #1a1a1a;
  }

  :global(body) {
    color: var(--fg-2);
    background-color: var(--bg-2);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    margin: 0;
  }

  :global(a) {
    color: #668cff;
    text-decoration: none;
    transition: .20s ease-in-out;
  }
  :global(a:hover) {
    color: #b3c6ff;
  }

  canvas {
    position: fixed;
    top: 0;
    left: 0;
  }

  .content {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    position: absolute;
  }

  .container {
    flex: 1 0 auto;
    margin: 2rem 0;

    p {
      text-align: justify;
    }

    > * {
      box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);
      background-color: rgba(0, 0, 0, 0.75);
      border-radius: .5rem;

      padding: 1rem 2rem;
    }
  }

  h1, h3 {
    color: var(--fg-1);
  }

  footer {
    display: flex;
    flex-shrink: 0;
    background-color: var(--bg-2);
    // clip-path: polygon(0 0, 50% 2rem, 100% 0, 100% 100%, 0px 100%);
    padding: 2rem;
  }

  @media only screen and (min-width: 600px) {
    .container {
      display: grid;
      grid-template-columns: 2rem repeat(3, 1fr) 2rem;
      grid-row-gap: 8rem;

      > * {
        grid-column-start: 2;
      }
    }

    header {
      grid-column: 2 / 5;
    }

    .wide {
      grid-column: 2 / 4;
    }

    footer {
      align-items: center;
      padding: 2rem 4rem;

      > h1 {
        padding-right: 4rem;
      }
    }
  }

  @media only screen and (max-width: 600px) {
    .container > * {
      margin: 2rem;
    }

    footer {
      flex-direction: column;
      text-align: center;

      > section {
        text-align: center;
      }
    }
  }
</style>
