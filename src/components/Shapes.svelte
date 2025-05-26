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

  const createHelix = (): THREE.Object3D => {
    const height = 2;
    const radius = 1;
    const path = new HelixCurve(radius, 4, height);
    const geometry = new THREE.TubeGeometry(path, 200, 0.1, 16, false);
    geometry.translate(0, 0, -(height / 2))

    const mesh = new THREE.Mesh(geometry, material);

    const startPoints = [];
    for (let i = 0; i <= geometry.parameters.radialSegments; i++) {
      startPoints.push(new THREE.Vector3().fromBufferAttribute(geometry.attributes.position, i));
    }
    const pointsStartGeometry = new THREE.BufferGeometry().setFromPoints(startPoints);
    const indexStart = [];
    for (let i = 0; i < pointsStartGeometry.attributes.position.count; i++) {
      indexStart.push(0, i, i + 1);
    }
    pointsStartGeometry.setIndex(indexStart);

    const shapeStart = new THREE.Mesh(pointsStartGeometry, material);

    const endPoints = [];
    endPoints.push(path.getPoint(1).sub(new THREE.Vector3(0, 0, height / 2)));
    for (let i = (geometry.parameters.radialSegments + 1) * geometry.parameters.tubularSegments; i < geometry.attributes.position.count; i++) {
      endPoints.push(new THREE.Vector3().fromBufferAttribute(geometry.attributes.position, i));
    }
    const pointsEndGeometry = new THREE.BufferGeometry().setFromPoints(endPoints);
    const indexEnd = [];
    for (let i = 1; i < pointsEndGeometry.attributes.position.count - 1; i++) {
      indexEnd.push(0, i + 1, i);
    }
    pointsEndGeometry.setIndex(indexEnd);

    const shapeEnd = new THREE.Mesh(pointsEndGeometry, material);

    const group = new THREE.Group();
    group.add(mesh);
    group.add(shapeStart);
    group.add(shapeEnd);

    group.rotation.x = 1.4;
    group.rotation.y = 1.8;
    return group;
  }

  const createCube = (): THREE.Object3D => {
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const mesh = new THREE.Mesh(geometry, material);
    return mesh;
  }

  const rotateHelix = (mesh: THREE.Object3D) => {
    mesh.rotateOnAxis(new THREE.Vector3(0, -0.1, 0.9), rotationSpeed);
  }

  const rotateCube = (mesh: THREE.Object3D) => {
    mesh.rotateOnAxis(new THREE.Vector3(0.5, 0.25, 0.25), rotationSpeed);
  }

  onMount(() => {
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(50, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.z = 6;

    const renderer = new THREE.WebGLRenderer({ canvas });
    renderer.setSize(window.innerWidth, window.innerHeight);

    const helix = createHelix();
    const cube = createCube();

    scene.add(helix);
    scene.add(cube);
    scene.background = new THREE.Color(0x030712);

    const animate = () => {
      rotateHelix(helix);
      rotateCube(cube);
      renderer.render(scene, camera);
    }
    renderer.setAnimationLoop(animate);

    const resize = () => {
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(window.innerWidth, window.innerHeight);

      const mx = 1;
      const my = 1;

      const vFOV = THREE.MathUtils.degToRad(camera.fov);
      const height = 2 * Math.tan(vFOV / 2) * camera.position.z;
      const width = height * camera.aspect;

      helix.position.set((width / 2) - mx, (height / 2) - my, -2);
      cube.position.set((-width / 2) + mx, (height / 3) - my, 0);
    }

    const scroll = () => {
      const scrollTop = window.scrollY;
      camera.rotation.x = -scrollTop * 0.001;
    }

    window.addEventListener('resize', resize);
    window.addEventListener('scroll', scroll);

    resize();
    scroll();
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
