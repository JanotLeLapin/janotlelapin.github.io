<script lang="ts">
  import { onMount } from 'svelte';

  import * as THREE from 'three';

  let el;

  onMount(() => {
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    const geometry = new THREE.BoxGeometry();
    const material = new THREE.MeshBasicMaterial({ color: 0xff0000, wireframe: true });
    const cube = new THREE.Mesh(geometry, material);
    let renderer;
    scene.add(cube);
    camera.position.z = 30;
    camera.position.y = -0.25;

    const animate = () => {
      requestAnimationFrame(animate);
      cube.rotation.y += 0.01;
      camera.position.z = camera.position.z < -10 ? 30 : camera.position.z - 0.1;
      renderer.render(scene, camera);
    };

    const resize = () => {
      renderer.setSize(window.innerWidth, window.innerHeight)
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
    };

    const createScene = (el) => {
      renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
      resize();
      animate();
    }

    createScene(el);

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
