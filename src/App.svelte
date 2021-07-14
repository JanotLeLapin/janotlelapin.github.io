<script lang="ts">
  import st from './assets/stone.jpg';

  import { onMount } from 'svelte';

  import { scene, camera, renderer, createScene, resize } from './scene';

  import * as THREE from 'three';

  let el: HTMLCanvasElement;

  onMount(() => {
    const stoneTexture = new THREE.TextureLoader().load(st);
    const stone = new THREE.Mesh(
      new THREE.BoxGeometry(),
      new THREE.MeshStandardMaterial({ map: stoneTexture })
    );

    const light = new THREE.PointLight(0xffffff);
    const ambient = new THREE.AmbientLight(0xffffff, 0.6);

    camera.rotation.x = -.5;
    camera.position.set(0, 1, 2);

    light.position.set(5, 2.5, 5);

    scene.add(stone, light, ambient);

    const animate = () => {
      requestAnimationFrame(animate);
      stone.rotation.y += 0.005;
      renderer.render(scene, camera);
    };

    createScene(el);
    animate();

    window.addEventListener('resize', resize);
  });
</script>

<main>
  <canvas bind:this={el}></canvas>
</main>

<style>
  canvas {
    position: fixed;
    top: 0;
    left: 0;
  }
</style>
