<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';

  let canvas: HTMLCanvasElement;

  onMount(() => {
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

    const renderer = new THREE.WebGLRenderer({ canvas });
    renderer.setSize(window.innerWidth, window.innerHeight);

    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.ShaderMaterial({
      vertexShader: `
        varying vec3 vPosition;

        void main() {
          vPosition = position;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,

      fragmentShader: `
        precision mediump float;
        varying vec3 vPosition;

        void main() {
          float mixFactor = (vPosition.x + vPosition.y) * 0.5 + 0.5;
          vec3 colorA = vec3(0.64, 0.70, 1.0);
          vec3 colorB = vec3(1.0, 0.63, 0.68);
          vec3 finalColor = mix(colorA, colorB, mixFactor);
          gl_FragColor = vec4(finalColor, 1.0);
        }
      `
    });

    const cube = new THREE.Mesh(geometry, material);
    scene.add(cube);

    camera.position.z = 5;

    const animate = () => {
      cube.rotation.x += 0.01;
      cube.rotation.y += 0.01;
      renderer.render(scene, camera);
    }
    renderer.setAnimationLoop(animate);
  })
</script>

<canvas bind:this={canvas} id="three"></canvas>

<style>
  canvas {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    z-index: -1;
  }
</style>
