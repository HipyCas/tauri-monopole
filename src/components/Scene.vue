<template>
  <div id="container">Loading</div>
</template>

<script>
import {
  Scene,
  PerspectiveCamera,
  WebGLRenderer,
  SphereGeometry,
  MeshBasicMaterial,
  Mesh,
  AxesHelper,
} from "three";
// import { TrackballControls } from "three/examples/jsm/controls/TrackballControls";
// import FlyControls from "../js/FlyControls";
// import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";

export default {
  props: ["data"],
  data: () => ({
    interruptAnimation: false,
    count: 0,
  }),
  setup() {
    // eslint-disable-next-line no-unused-vars
    const scene = new Scene();
    // eslint-disable-next-line no-unused-vars
    const camera = new PerspectiveCamera(
      75, // FOV (degrees)
      window.innerWidth / window.innerHeight, // Aspect ratio
      0.1, // near and
      1000 // far clipping plane
    );
    const renderer = new WebGLRenderer();

    const geometry = new SphereGeometry(0.1, 32, 32);
    const material = new MeshBasicMaterial({ color: 0x00ff00 });
    const monopole = new Mesh(geometry, material);
    scene.add(monopole);

    const axesHelper = new AxesHelper(20); // Size, default 1
    scene.add(axesHelper);

    camera.position.z = 5;
    camera.position.y = 5;
    camera.position.x = 5;
    camera.lookAt(0, 0, 0);

    return {
      scene,
      camera,
      renderer,
      monopole,
      controls: null,
    };
  },
  mounted() {
    this.renderer.setSize(window.innerWidth, window.innerHeight);

    // console.log(document);

    // this.controls = new FlyControls(this.camera, this.renderer.domElement);
    // this.controls = new TrackballControls(
    //   this.camera,
    //   this.renderer.domElement
    // );
    // this.controls.rotateSpeed = 1.0;
    // this.controls.zoomSpeed = 1.2;
    // this.controls.panSpeed = 0.8;
    // this.controls.noZoom = false;
    // this.controls.noPan = false;
    // this.controls.staticMoving = true;
    // this.controls.dynamicDampingFactor = 0.3;
    // console.log(this.controls);

    document.querySelector("#container").innerHTML = "";
    document.querySelector("#container").appendChild(this.renderer.domElement);

    // this.animate();

    setInterval(this.updateCamera, 100); // TODO Move inside animate? Maybe you can do it as if ellapsed time is multiple of 100
  },
  methods: {
    animate() {
      if (this.interruptAnimation) {
        this.interruptAnimation = false;
        cancelAnimationFrame(this.animate);
        return;
      }
      // console.info("animating");
      this.loadData();
      requestAnimationFrame(this.animate);
      this.renderer.render(this.scene, this.camera);
      // this.controls.update();
    },
    render() {
      // console.info("rendering");
      this.loadData();
      this.renderer.render(this.scene, this.camera);
      // this.controls.update(1);
    },
    loadData() {
      console.log("Loading points from data:");
      console.log(this.$props.data);
      this.$props.data.forEach((item) => {
        this.scene.add(this.createPoint(item.position));
      });
    },
    createPoint(pt) {
      // console.log(pt);
      const geometry = new SphereGeometry(0.05, 32, 32);
      const material = new MeshBasicMaterial({ color: 0xff0000 });
      const point = new Mesh(geometry, material);

      point.position.x = pt.x;
      point.position.y = pt.y;
      point.position.z = pt.z;

      return point;
    },
    updateCamera() {
      if (this.interruptAnimation) {
        clearInterval(this.updateCamera);
        return;
      }
      if (this.$props.data[this.count] === undefined) return;
      console.info(
        `Looking at (${this.$props.data[this.count].position.x}, ${
          this.$props.data[this.count].position.y
        }, ${this.$props.data[this.count].position.z})`
      );
      this.camera.lookAt(
        this.$props.data[this.count].position.x,
        this.$props.data[this.count].position.y,
        this.$props.data[this.count].position.z
      );
      this.count += 1;
      if (this.count === this.$props.data.length) {
        this.count = 0;
      }
    },
  },
};
</script>

<style lang="stylus" scoped>
#container
  position absolute
  height 100vh
  width 100vw
  z-index 0
</style>
