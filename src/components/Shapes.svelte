<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';

  let canvas: HTMLCanvasElement;

  const rotationSpeed = 0.008;

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

  class HelixCurve extends THREE.Curve<THREE.Vector3> {
    private radius: number;
    private turns: number;
    private height: number;

    constructor(radius = 2, turns = 5, height = 10) {
      super();
      this.radius = radius;
      this.turns = turns;
      this.height = height;
    }

    getPoint(t: number) {
      const angle = 2 * Math.PI * this.turns * t;
      const x = this.radius * Math.cos(angle);
      const y = this.radius * Math.sin(angle)
      const z = this.height * t;
      return new THREE.Vector3(x, y, z);
    }
  }

  const createHelix = (): THREE.Mesh => {
    const path = new HelixCurve(0.4, 2, 1);
    const geometry = new THREE.TubeGeometry(path, 200, 0.1, 16, false);
    const mesh = new THREE.Mesh(geometry, material);
    mesh.rotation.x = 1.4;
    mesh.rotation.y = 1.8;
    return mesh;
  }

  const createCube = (): THREE.Mesh => {
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const mesh = new THREE.Mesh(geometry, material);
    mesh.position.x = 2;
    return mesh;
  }

  const rotateHelix = (mesh: THREE.Mesh) => {
    mesh.rotateOnAxis(new THREE.Vector3(0, 0, 1), rotationSpeed);
  }

  const rotateCube = (mesh: THREE.Mesh) => {
    mesh.rotateOnAxis(new THREE.Vector3(1, 1, 0), rotationSpeed / 2);
  }

  onMount(() => {
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

    const renderer = new THREE.WebGLRenderer({ canvas });
    renderer.setSize(window.innerWidth, window.innerHeight);

    const helix = createHelix();
    const cube = createCube();
    scene.add(helix);
    scene.add(cube);

    camera.position.z = 6;

    const animate = () => {
      rotateHelix(helix);
      rotateCube(cube);
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
