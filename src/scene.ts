import * as THREE from 'three';

export const scene = new THREE.Scene();
export const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

export let renderer: THREE.WebGLRenderer;

export const resize = () => {
  renderer.setSize(window.innerWidth, window.innerHeight);
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
}

export const createScene = (el: HTMLCanvasElement) => {
  renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
  resize();
}
