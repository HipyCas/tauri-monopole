<template>
  <scene :data="results" ref="scene" />
  <actions @run="run" @animate="animate" @stop-animate="stopAnimate" />
</template>

<script>
/* eslint-disable no-unused-vars */
import { invoke } from "@tauri-apps/api/tauri";

// @ is an alias to /src
//import HelloWorld from "@/components/HelloWorld.vue";
//import Settings from "@/components/Settings.vue";
import Actions from "@/components/Actions.vue";
import Scene from "@/components/Scene.vue";
// import Settings from "@/components/Settings.vue";

export default {
  name: "Home",
  components: { Actions, Scene },
  data: () => ({
    results: [],
  }),
  mounted() {
    //this.exec();
  },
  methods: {
    async exec() {
      return new Promise((resolve, reject) => {
        if (window.rpc === undefined) {
          alert(
            "The application is not running as native application, therefore the simulation cannot be run."
          );
          //reject("not running as desktop application");
          resolve([]);
        }
        // reject("Test error");
        invoke("exec", this.$store.state.variables)
          .then((res) => resolve(res))
          .catch((err) => reject(err));
      });
    },
    run() {
      this.exec()
        .then((res) => {
          (this.results = res), this.$refs.scene.render();
        })
        .catch((err) => {
          //throw err;
          invoke("error", {
            title: "Error running simulation",
            msg: `Received the following error message from the backend code calculation: ${err}`,
          });
        });
    },
    animate() {
      this.exec()
        .then((res) => {
          (this.results = res), this.$refs.scene.animate();
        })
        .catch((err) => {
          // throw err;
          invoke("error", {
            title: "Error running simulation",
            msg: `Received the following error message from the backend code calculation: ${err}`,
          });
        });
    },
    stopAnimate() {
      this.$refs.scene.interruptAnimation = true;
    },
  },
};
</script>
