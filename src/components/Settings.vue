<template>
  <section id="settings-pane">
    <h1>Settings</h1>
    <button @click="loadConfig">Load Config</button>
    <button @click="saveConfig">Save Config</button>
    <div class="variable">
      <h3>Number of iterations</h3>
      <input
        type="number"
        v-model="count"
        class="single-value-input"
        @input="saveCount"
      />
      <p class="helper-text">
        The number of poits that will be calculated, not including initial
        position
      </p>
    </div>
    <div class="variable">
      <h3>Initial velocity</h3>
      <input
        type="number"
        v-model="velocityX"
        class="single-value-input"
        @input="saveVelocity"
      />
      <input
        type="number"
        v-model="velocityY"
        class="single-value-input"
        @input="saveVelocity"
      />
      <input
        type="number"
        v-model="velocityZ"
        class="single-value-input"
        @input="saveVelocity"
      />
    </div>
    <div class="variable">
      <h3>Initial position</h3>
      <input
        type="number"
        v-model="positionX"
        class="single-value-input"
        @input="savePosition"
      />
      <input
        type="number"
        v-model="positionY"
        class="single-value-input"
        @input="savePosition"
      />
      <input
        type="number"
        v-model="positionZ"
        class="single-value-input"
        @input="savePosition"
      />
    </div>
    <div class="variable">
      <h3>Mass</h3>
      <input
        type="number"
        v-model="mass"
        class="single-value-input"
        @input="saveMass"
        min="0"
      />
    </div>
    <div class="variable">
      <h3>Charge</h3>
      <input
        type="number"
        v-model="charge"
        class="single-value-input"
        @input="saveCharge"
      />
    </div>
    <div class="variable">
      <h3>Field intensity</h3>
      <input
        type="number"
        v-model="intensity"
        class="single-value-input"
        @input="saveIntensity"
      />
      <p class="helper-text">
        The magnetic field is calculated by Intensity / (Radius ^ 2), sign
        matters
      </p>
    </div>
  </section>
  <div id="loading" v-if="loading">Loading...</div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
  data: () => ({
    loading: true,
    count: 0,
    velocityX: 0,
    velocityY: 0,
    velocityZ: 0,
    positionX: 0,
    positionY: 0,
    positionZ: 0,
    mass: 0,
    charge: 0,
    intensity: 0,
  }),
  mounted() {
    this.count = this.$store.state.variables.count;
    this.velocityX = this.$store.state.variables.velocity.x;
    this.velocityY = this.$store.state.variables.velocity.y;
    this.velocityZ = this.$store.state.variables.velocity.z;
    this.positionX = this.$store.state.variables.position.x;
    this.positionY = this.$store.state.variables.position.y;
    this.positionZ = this.$store.state.variables.position.z;
    this.mass = this.$store.state.variables.mass;
    this.charge = this.$store.state.variables.charge;
    this.intensity = this.$store.state.variables.intensity;
    // TODO time_ms?

    this.loading = false;
  },
  methods: {
    saveCount() {
      this.loading = true;
      console.log("saving count");
      this.$store.commit("setCount", this.count);
      this.loading = false;
    },
    saveVelocity() {
      this.loading = true;
      console.log("saving velocity");
      this.$store.commit("setVelocity", {
        x: this.velocityX,
        y: this.velocityY,
        z: this.velocityZ,
      });
      this.loading = false;
    },
    savePosition() {
      this.loading = true;
      console.log("saving position");
      this.$store.commit("setPosition", {
        x: this.positionX,
        y: this.positionY,
        z: this.positionZ,
      });
      this.loading = false;
    },
    saveMass() {
      this.loading = true;
      console.log("saving mass");
      this.$store.commit("setMass", this.mass);
      this.loading = false;
    },
    saveCharge() {
      this.loading = true;
      console.log("saving charge");
      this.$store.commit("setCharge", this.charge);
      this.loading = false;
    },
    saveIntensity() {
      this.loading = true;
      console.log("saving intensity");
      this.$store.commit("setIntensity", this.intensity);
      this.loading = false;
    },
    loadConfig() {
      invoke("load_config")
        .then((res) => {
          console.log(res);
          this.$store.commit("updateVariables", res);
        })
        .catch((err) =>
          invoke("error", {
            title: "Error while loading config",
            msg: `${err}`,
          })
        );
    },
    saveConfig() {
      console.log(this.$store.state.variables);
      console.log("hi");
      invoke("save_config", { config: this.$store.state.variables ?? {} })
        .then((res) => console.log(res))
        .catch((err) =>
          invoke("error", {
            title: "Error while saving config",
            msg: err,
          })
        );
    },
  },
};
</script>

<style lang="stylus" scoped>
#loading
  height 100vh
  width 100vw
  position absolute
  z-index 100
  display flex
  justify-content center
  align-items center
  background-color #111111 // TODO Move into var, move all colors together in a file and @import/@require
  color white
  opacity 0.5

#settings-pane
  background-color white
  min-height calc(100vh - 4em)
  width calc(100vw - 4em)
  position absolute
  z-index 30
  padding 2em

.variable
  display flex
  flex-direction row
  align-items center

  h3
    margin-right: 1em

  .single-value-input
    width 3em
    margin-right 0.5em
    height 1em

  .helper-text
    margin-left 0.75em
    font-size 12px
    color #111111
</style>
